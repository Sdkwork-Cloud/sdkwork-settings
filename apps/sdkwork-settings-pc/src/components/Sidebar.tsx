/**
 * 左侧分区导航组件。
 *
 * 包含三个分区:
 * - 用户偏好:外观、语言、通知、隐私
 * - 租户配置:品牌、安全策略、功能开关(标记为"管理员")
 * - 系统设置:全局参数(标记为"系统管理员")
 */

export type SectionKey = 'preference' | 'tenant' | 'system';

export type SubSectionKey =
  | 'appearance'
  | 'language'
  | 'notifications'
  | 'privacy'
  | 'brand'
  | 'security'
  | 'features'
  | 'global';

interface NavItem {
  key: SubSectionKey;
  label: string;
}

interface NavSection {
  key: SectionKey;
  label: string;
  badge?: string;
  items: NavItem[];
}

// 分区导航结构定义
const NAV_SECTIONS: NavSection[] = [
  {
    key: 'preference',
    label: '用户偏好',
    items: [
      { key: 'appearance', label: '外观' },
      { key: 'language', label: '语言' },
      { key: 'notifications', label: '通知' },
      { key: 'privacy', label: '隐私' },
    ],
  },
  {
    key: 'tenant',
    label: '租户配置',
    badge: '管理员',
    items: [
      { key: 'brand', label: '品牌' },
      { key: 'security', label: '安全策略' },
      { key: 'features', label: '功能开关' },
    ],
  },
  {
    key: 'system',
    label: '系统设置',
    badge: '系统管理员',
    items: [{ key: 'global', label: '全局参数' }],
  },
];

interface SidebarProps {
  activeSection: SectionKey;
  activeSubSection: SubSectionKey;
  onSelect: (section: SectionKey, subSection: SubSectionKey) => void;
}

export function Sidebar({ activeSection, activeSubSection, onSelect }: SidebarProps) {
  return (
    <aside className="settings-scrollbar flex w-60 shrink-0 flex-col gap-5 overflow-y-auto border-r border-gray-200 bg-white px-3 py-5 dark:border-neutral-800 dark:bg-neutral-900">
      {NAV_SECTIONS.map((section) => (
        <nav key={section.key} className="flex flex-col gap-0.5">
          <div className="flex items-center gap-2 px-3 pb-1.5">
            <h2 className="text-[11px] font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">
              {section.label}
            </h2>
            {section.badge && (
              <span className="rounded-full bg-blue-50 px-1.5 py-px text-[10px] font-medium text-blue-600 dark:bg-blue-500/15 dark:text-blue-300">
                {section.badge}
              </span>
            )}
          </div>

          {section.items.map((item) => {
            const isActive = section.key === activeSection && item.key === activeSubSection;
            return (
              <button
                key={item.key}
                type="button"
                onClick={() => onSelect(section.key, item.key)}
                className={
                  'flex items-center rounded-lg px-3 py-1.5 text-left text-[13px] transition-colors ' +
                  (isActive
                    ? 'bg-blue-50 font-medium text-blue-700 dark:bg-blue-500/15 dark:text-blue-300'
                    : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900 dark:text-gray-300 dark:hover:bg-neutral-800 dark:hover:text-gray-100')
                }
              >
                {item.label}
              </button>
            );
          })}
        </nav>
      ))}
    </aside>
  );
}
