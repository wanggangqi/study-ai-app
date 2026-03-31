import React, { useState, useCallback } from 'react';
import { Button } from '../common/Button';
import { useCourseStore } from '../../stores/courseStore';
import { generateStructuredExercise } from '../../hooks/useAI';
import type { Exercise, ExerciseOption } from '../../types';
import './ExercisePanel.css';

interface ExercisePanelProps {
  lessonId: string;
  lessonContent: string | null;
}

/**
 * 练习题面板组件
 * 支持选择题和填空题，展示练习题列表，提交答案，显示分析结果
 * 得分超过 60% 自动标记课时为完成
 */
export const ExercisePanel: React.FC<ExercisePanelProps> = ({
  lessonId,
  lessonContent,
}) => {
  const { exercises, setExercises, submitExerciseAnswer, updateLessonStatus } = useCourseStore();
  const [isLoading, setIsLoading] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [currentAnswers, setCurrentAnswers] = useState<Record<string, string>>({});
  const [showResults, setShowResults] = useState(false);

  // 过滤当前课时的练习题
  const lessonExercises = exercises.filter((ex) => ex.lessonId === lessonId);

  // 生成练习题
  const handleGenerateExercises = useCallback(async () => {
    if (!lessonContent) {
      setError('课件内容为空，请先加载课件');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const result = await generateStructuredExercise(lessonId, lessonContent);

      if (result.success && result.data) {
        // 将后端返回的结构转换为前端类型
        const frontExercises: Exercise[] = result.data.map((ex) => ({
          id: ex.id,
          lessonId: lessonId,
          question: ex.question,
          options: ex.options.map((opt) => ({
            id: opt.id,
            label: opt.label,
            content: opt.content,
          })),
          correctAnswer: ex.correct_answer,
          explanation: ex.explanation,
        }));
        setExercises(frontExercises);
        setCurrentAnswers({});
        setShowResults(false);
      } else {
        setError(result.error || '生成练习题失败');
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  }, [lessonId, lessonContent, setExercises]);

  // 选择答案
  const handleSelectAnswer = useCallback((exerciseId: string, answer: string) => {
    setCurrentAnswers((prev) => ({
      ...prev,
      [exerciseId]: answer,
    }));
  }, []);

  // 填写填空题答案
  const handleFillAnswer = useCallback((exerciseId: string, answer: string) => {
    setCurrentAnswers((prev) => ({
      ...prev,
      [exerciseId]: answer,
    }));
  }, []);

  // 提交答案
  const handleSubmitAnswers = useCallback(() => {
    // 防止重复提交
    if (isSubmitting) return;

    setIsSubmitting(true);

    // 遍历所有练习题，提交答案
    lessonExercises.forEach((exercise) => {
      const answer = currentAnswers[exercise.id];
      if (answer !== undefined) {
        submitExerciseAnswer(exercise.id, answer);
      }
    });
    setShowResults(true);

    // 计算正确率，得分超过 60% 自动标记课时为完成
    const correctCount = lessonExercises.filter((ex) => {
      const answer = currentAnswers[ex.id];
      return answer !== undefined && answer === ex.correctAnswer;
    }).length;
    const totalCount = lessonExercises.length;
    const accuracy = totalCount > 0 ? Math.round((correctCount / totalCount) * 100) : 0;

    if (accuracy >= 60) {
      updateLessonStatus(lessonId, 'completed');
    }

    setIsSubmitting(false);
  }, [lessonExercises, currentAnswers, submitExerciseAnswer, lessonId, updateLessonStatus, isSubmitting]);

  // 判断是否为选择题（有选项）或填空题
  const isChoiceExercise = (exercise: Exercise): boolean => {
    return exercise.options && exercise.options.length > 0;
  };

  // 渲染单个练习题
  const renderExercise = (exercise: Exercise, index: number) => {
    const hasAnswered = exercise.userAnswer !== undefined;
    const isCorrect = exercise.isCorrect;
    const userAnswer = currentAnswers[exercise.id] || exercise.userAnswer;
    const showExplanation = showResults && hasAnswered;

    return (
      <div
        key={exercise.id}
        className={`exercise-item ${hasAnswered ? (isCorrect ? 'correct' : 'incorrect') : ''}`}
      >
        <div className="exercise-question">
          <span className="exercise-number">{index + 1}.</span>
          <span className="exercise-text">{exercise.question}</span>
        </div>

        {isChoiceExercise(exercise) ? (
          <div className="exercise-options">
            {exercise.options.map((option: ExerciseOption) => {
              const isSelected = userAnswer === option.id;
              const isCorrectOption = option.id === exercise.correctAnswer;
              let optionClass = 'exercise-option';

              if (showExplanation) {
                if (isCorrectOption) {
                  optionClass += ' correct-option';
                } else if (isSelected && !isCorrect) {
                  optionClass += ' incorrect-option';
                }
              } else if (isSelected) {
                optionClass += ' selected';
              }

              return (
                <label
                  key={option.id}
                  className={optionClass}
                  onClick={() => !hasAnswered && handleSelectAnswer(exercise.id, option.id)}
                >
                  <span className="option-radio">
                    {isSelected ? '●' : '○'}
                  </span>
                  <span className="option-label">{option.label}.</span>
                  <span className="option-content">{option.content}</span>
                </label>
              );
            })}
          </div>
        ) : (
          <div className="exercise-fill">
            <input
              type="text"
              className={`fill-input ${showExplanation ? (isCorrect ? 'correct' : 'incorrect') : ''}`}
              value={userAnswer || ''}
              onChange={(e) => handleFillAnswer(exercise.id, e.target.value)}
              disabled={hasAnswered}
              placeholder="请输入答案..."
            />
            {showExplanation && !isCorrect && (
              <div className="correct-answer-hint">
                正确答案: {exercise.correctAnswer}
              </div>
            )}
          </div>
        )}

        {showExplanation && exercise.explanation && (
          <div className={`exercise-explanation ${isCorrect ? 'correct' : 'incorrect'}`}>
            <div className="explanation-header">
              {isCorrect ? '✓ 回答正确' : '✗ 回答错误'}
            </div>
            <div className="explanation-content">{exercise.explanation}</div>
          </div>
        )}
      </div>
    );
  };

  // 计算正确率
  const correctCount = lessonExercises.filter((ex) => ex.isCorrect).length;
  const totalCount = lessonExercises.length;
  const accuracy = totalCount > 0 ? Math.round((correctCount / totalCount) * 100) : 0;

  return (
    <div className="exercise-panel">
      <div className="exercise-header">
        <h3 className="exercise-title">练习题</h3>
        <Button
          variant="primary"
          size="sm"
          onClick={handleGenerateExercises}
          disabled={isLoading || !lessonContent}
        >
          {isLoading ? '生成中...' : '生成练习题'}
        </Button>
      </div>

      {error && <div className="exercise-error">{error}</div>}

      {lessonExercises.length === 0 ? (
        <div className="exercise-empty">
          <div className="empty-icon">📝</div>
          <p className="empty-text">暂无练习题</p>
          <p className="empty-hint">点击上方按钮生成练习题</p>
        </div>
      ) : (
        <>
          <div className="exercise-list">
            {lessonExercises.map((exercise, index) => renderExercise(exercise, index))}
          </div>

          {showResults && totalCount > 0 && (
            <div className="exercise-summary">
              <div className="summary-content">
                <span className="summary-label">正确率:</span>
                <span className={`summary-value ${accuracy >= 60 ? 'good' : 'poor'}`}>
                  {accuracy}%
                </span>
                <span className="summary-detail">
                  ({correctCount}/{totalCount})
                </span>
              </div>
            </div>
          )}

          {!showResults && (
            <div className="exercise-actions">
              <Button
                variant="primary"
                size="sm"
                onClick={handleSubmitAnswers}
                disabled={isSubmitting || Object.keys(currentAnswers).length < totalCount}
              >
                {isSubmitting ? '提交中...' : '提交答案'}
              </Button>
            </div>
          )}
        </>
      )}
    </div>
  );
};