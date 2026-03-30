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
    <div className="w-48 bg-bg-secondary min-h-screen p-4 flex flex-col gap-2">
      {items.map((item) => (
        <button
          key={item.path}
          onClick={() => onNavigate(item.path)}
          className={`flex items-center gap-3 px-4 py-3 rounded-md text-left transition-colors ${
            activePath === item.path
              ? 'bg-white text-primary font-medium shadow-sm'
              : 'text-text-secondary hover:bg-white/50'
          }`}
        >
          <span className="text-lg">{item.icon}</span>
          <span className="flex-1">{item.label}</span>
          {item.badge && (
            <span className="bg-accent text-text-primary text-xs px-2 py-0.5 rounded-full">
              {item.badge}
            </span>
          )}
        </button>
      ))}
    </div>
  );
};
