import React from 'react';

interface NavItem {
  icon: string;
  label: string;
  path: string;
  badge?: string;
}

interface SidebarProps {
  items: NavItem[];
  activePath: string;
  onNavigate: (path: string) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ items, activePath, onNavigate }) => {
  return (
    <aside className="w-48 h-full bg-[#f5ebe0] p-4 flex flex-col gap-2 fixed left-0 top-0 overflow-y-auto">
      {/* Logo 区域 */}
      <div className="mb-4 px-4 py-3">
        <div className="flex items-center gap-2">
          <span className="text-xl">&#128218;</span>
          <span className="font-bold text-[#588157]">智学伴侣</span>
        </div>
      </div>

      {/* 导航项 */}
      {items.map((item) => (
        <button
          key={item.path}
          onClick={() => onNavigate(item.path)}
          className={`flex items-center gap-3 px-4 py-3 rounded-md text-left transition-colors ${
            activePath === item.path
              ? 'bg-white text-[#588157] font-medium shadow-sm'
              : 'text-[#666666] hover:bg-white/50'
          }`}
        >
          <span className="text-lg">{item.icon}</span>
          <span className="flex-1">{item.label}</span>
          {item.badge && (
            <span className="bg-[#e9c46a] text-[#333333] text-xs px-2 py-0.5 rounded-full">
              {item.badge}
            </span>
          )}
        </button>
      ))}
    </aside>
  );
};
