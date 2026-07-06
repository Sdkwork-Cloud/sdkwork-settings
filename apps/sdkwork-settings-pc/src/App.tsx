import { useEffect, useState } from 'react';
import { Sidebar, type SectionKey, type SubSectionKey } from './components/Sidebar';
import { PreferencePanel } from './components/PreferencePanel';

/**
 * SDKWork Settings PC 应用主组件。
 *
 * 布局:顶部导航栏 + 左侧分区导航 + 右侧内容区。
 * 视觉风格对齐 Chrome / Edge / Arc 等浏览器设置界面。
 */
export default function App() {
  // 当前选中的分区
  const [section, setSection] = useState<SectionKey>('preference');
  // 当前选中的子项(用于在面板内部定位)
  const [subSection, setSubSection] = useState<SubSectionKey>('appearance');
  // 深色模式开关,通过 html.dark 控制
  const [darkMode, setDarkMode] = useState(false);

  // 同步深色模式 class 到 <html>
  useEffect(() => {
    const root = document.documentElement;
    if (darkMode) {
      root.classList.add('dark');
    } else {
      root.classList.remove('dark');
    }
  }, [darkMode]);

  return (
    <div className="flex h-full flex-col bg-gray-50 dark:bg-neutral-900">
      {/* 顶部导航栏 */}
      <header className="flex h-14 shrink-0 items-center justify-between border-b border-gray-200 bg-white px-6 dark:border-neutral-800 dark:bg-neutral-900">
        <div className="flex items-center gap-2.5">
          <div className="flex h-7 w-7 items-center justify-center rounded-md bg-gradient-to-br from-blue-500 to-blue-600 text-sm font-bold text-white shadow-sm">
            S
          </div>
          <span className="text-[15px] font-semibold text-gray-800 dark:text-gray-100">
            SDKWork Settings
          </span>
        </div>

        <div className="flex items-center gap-3">
          {/* 主题切换 */}
          <button
            type="button"
            onClick={() => setDarkMode((prev) => !prev)}
            className="flex h-8 w-8 items-center justify-center rounded-full text-gray-500 transition-colors hover:bg-gray-100 hover:text-gray-700 dark:text-gray-400 dark:hover:bg-neutral-800 dark:hover:text-gray-200"
            title={darkMode ? '切换到浅色模式' : '切换到深色模式'}
            aria-label="切换主题"
          >
            {darkMode ? <SunIcon /> : <MoonIcon />}
          </button>

          <div className="h-5 w-px bg-gray-200 dark:bg-neutral-700" />

          {/* 用户信息 */}
          <div className="flex items-center gap-2">
            <div className="flex h-8 w-8 items-center justify-center rounded-full bg-gradient-to-br from-emerald-400 to-blue-500 text-xs font-semibold text-white">
              示
            </div>
            <div className="hidden flex-col leading-tight sm:flex">
              <span className="text-[13px] font-medium text-gray-800 dark:text-gray-100">
                示例用户
              </span>
              <span className="text-[11px] text-gray-500 dark:text-gray-400">
                demo@sdkwork.com
              </span>
            </div>
          </div>
        </div>
      </header>

      {/* 主体:左侧导航 + 右侧内容 */}
      <div className="flex min-h-0 flex-1">
        <Sidebar
          activeSection={section}
          activeSubSection={subSection}
          onSelect={(nextSection, nextSub) => {
            setSection(nextSection);
            setSubSection(nextSub);
          }}
        />

        <main className="settings-scrollbar min-h-0 flex-1 overflow-y-auto">
          <div className="mx-auto w-full max-w-3xl px-8 py-8">
            {section === 'preference' && (
              <PreferencePanel subSection={subSection} onSubSectionChange={setSubSection} />
            )}
            {section === 'tenant' && <PlaceholderPanel title="租户配置" />}
            {section === 'system' && <PlaceholderPanel title="系统设置" />}
          </div>
        </main>
      </div>
    </div>
  );
}

/** 占位面板:用于尚未实现的分区(租户配置 / 系统设置)。 */
function PlaceholderPanel({ title }: { title: string }) {
  return (
    <section>
      <h1 className="text-2xl font-semibold text-gray-800 dark:text-gray-100">{title}</h1>
      <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">
        该分区将在后续阶段实现,届时将接入对应的 SDK 与后端配置接口。
      </p>
      <div className="mt-6 rounded-xl border border-dashed border-gray-300 bg-white/60 p-10 text-center text-sm text-gray-400 dark:border-neutral-700 dark:bg-neutral-800/40 dark:text-gray-500">
        敬请期待
      </div>
    </section>
  );
}

function SunIcon() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <circle cx="12" cy="12" r="4" />
      <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" />
    </svg>
  );
}

function MoonIcon() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
    </svg>
  );
}
