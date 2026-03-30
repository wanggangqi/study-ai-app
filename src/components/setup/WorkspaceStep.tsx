import React, { useState } from 'react';
import { useConfigStore } from '../../stores/configStore';
import { Button } from '../common/Button';
import { SetupStepProps } from './SetupWizard';
import { open } from '@tauri-apps/plugin-dialog';

export const WorkspaceStep: React.FC<SetupStepProps> = ({ onNext, onBack }) => {
  const [workspacePath, setWorkspacePath] = useState('');
  const [error, setError] = useState('');
  const { setConfig } = useConfigStore();

  const handleSelectFolder = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '选择工作空间目录',
      });

      if (selected && typeof selected === 'string') {
        setWorkspacePath(selected);
        setError('');
      }
    } catch (err) {
      console.error('Failed to select folder:', err);
      setError('选择目录失败');
    }
  };

  const handleSubmit = () => {
    if (!workspacePath.trim()) {
      return;
    }

    setConfig({ workspacePath });
    onNext();
  };

  return (
    <div>
      <div className="text-center mb-6">
        <div className="text-4xl mb-4">📁</div>
        <h2 className="text-xl font-bold mb-2">设置工作空间</h2>
        <p className="text-text-secondary">选择本地工作空间目录，用于存储课程文件</p>
      </div>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            工作空间路径
          </label>
          <div className="flex gap-2">
            <input
              type="text"
              className="flex-1 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary"
              value={workspacePath}
              onChange={(e) => setWorkspacePath(e.target.value)}
              placeholder="选择或输入目录路径"
              readOnly
            />
            <Button onClick={handleSelectFolder} variant="outline">
              浏览...
            </Button>
          </div>
        </div>
        {error && <p className="text-sm text-red-500">{error}</p>}
        <p className="text-xs text-text-muted">
          课程文件将保存在此目录下，每个课程会有独立的子文件夹。
        </p>
      </div>

      <div className="flex justify-between mt-6">
        <Button variant="outline" onClick={onBack}>
          上一步
        </Button>
        <Button onClick={handleSubmit} disabled={!workspacePath.trim()}>
          下一步
        </Button>
      </div>
    </div>
  );
};