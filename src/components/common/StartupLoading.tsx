import React from 'react';

export const StartupLoading: React.FC = () => {
  return (
    <div className="fixed inset-0 flex items-center justify-center bg-[#fefae0]">
      <div className="text-center">
        {/* Logo 动画 */}
        <div className="relative mb-8">
          <div className="w-20 h-20 mx-auto relative">
            {/* 外圈旋转 */}
            <div className="absolute inset-0 rounded-full border-4 border-[#588157]/20" />
            <div
              className="absolute inset-0 rounded-full border-4 border-transparent border-t-[#588157]"
              style={{
                animation: 'spin 1.5s linear infinite',
              }}
            />
            {/* 内圈脉冲 */}
            <div
              className="absolute inset-2 rounded-full bg-[#d4a373]/30"
              style={{
                animation: 'pulse 2s ease-in-out infinite',
              }}
            />
            {/* 中心图标 */}
            <div className="absolute inset-0 flex items-center justify-center">
              <span className="text-3xl">&#128218;</span>
            </div>
          </div>
        </div>

        {/* 标题 */}
        <h1 className="text-2xl font-bold text-[#588157] mb-2">智学伴侣</h1>

        {/* 加载文字 */}
        <p className="text-[#666666] text-sm animate-pulse">正在初始化...</p>
      </div>

      {/* 底部装饰 */}
      <div className="absolute bottom-8 left-0 right-0 text-center">
        <div className="flex justify-center gap-2">
          <div
            className="w-2 h-2 rounded-full bg-[#588157]"
            style={{ animation: 'bounce 1s ease-in-out infinite', animationDelay: '0ms' }}
          />
          <div
            className="w-2 h-2 rounded-full bg-[#d4a373]"
            style={{ animation: 'bounce 1s ease-in-out infinite', animationDelay: '150ms' }}
          />
          <div
            className="w-2 h-2 rounded-full bg-[#e9c46a]"
            style={{ animation: 'bounce 1s ease-in-out infinite', animationDelay: '300ms' }}
          />
        </div>
      </div>
    </div>
  );
};