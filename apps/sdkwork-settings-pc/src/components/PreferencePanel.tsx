import { useState, type ReactNode } from 'react';
import type { SubSectionKey } from './Sidebar';

/**
 * 用户偏好面板。
 *
 * 包含四组设置:外观、语言、通知、隐私。
 * 当前阶段使用本地 React state 管理,API 调用使用占位函数;
 * 实际的 SDK 集成与后端持久化将在后续阶段接入。
 */

interface PreferencePanelProps {
  subSection: SubSectionKey;
  onSubSectionChange: (sub: SubSectionKey) => void;
}

// 占位 API:后续替换为真实 SDK 调用
async function savePreferences(payload: Record<string, unknown>): Promise<void> {
  // TODO: 接入 SDKWork Settings App SDK 后替换为真实持久化调用
  console.debug('[preferences] save (placeholder)', payload);
}

export function PreferencePanel({
  subSection,
  onSubSectionChange,
}: PreferencePanelProps) {
  return (
    <section>
      <h1 className="text-2xl font-semibold text-gray-800 dark:text-gray-100">用户偏好</h1>
      <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">
        管理你的个性化设置。更改会保存到当前账户,并在所有设备上同步。
      </p>

      {/* 子项快捷切换(横向标签) */}
      <div className="mt-5 flex flex-wrap gap-1 rounded-lg bg-gray-100 p-1 dark:bg-neutral-800">
        {(
          [
            ['appearance', '外观'],
            ['language', '语言'],
            ['notifications', '通知'],
            ['privacy', '隐私'],
          ] as const
        ).map(([key, label]) => (
          <button
            key={key}
            type="button"
            onClick={() => onSubSectionChange(key)}
            className={
              'flex-1 rounded-md px-3 py-1.5 text-[13px] font-medium transition-colors ' +
              (subSection === key
                ? 'bg-white text-blue-700 shadow-sm dark:bg-neutral-700 dark:text-blue-300'
                : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200')
            }
          >
            {label}
          </button>
        ))}
      </div>

      <div className="mt-6">
        {subSection === 'appearance' && <AppearanceGroup />}
        {subSection === 'language' && <LanguageGroup />}
        {subSection === 'notifications' && <NotificationsGroup />}
        {subSection === 'privacy' && <PrivacyGroup />}
      </div>
    </section>
  );
}

/* ---------------- 外观 ---------------- */

type ThemeMode = 'light' | 'dark';
type Density = 'comfortable' | 'compact';
type FontSize = 'small' | 'medium' | 'large';

function AppearanceGroup() {
  const [theme, setTheme] = useState<ThemeMode>('light');
  const [fontSize, setFontSize] = useState<FontSize>('medium');
  const [density, setDensity] = useState<Density>('comfortable');

  return (
    <SettingCard
      title="外观"
      description="自定义界面的主题、字号与显示密度。"
      onSave={() => savePreferences({ theme, fontSize, density })}
    >
      <SettingRow label="主题" description="选择浅色或深色主题。">
        <SegmentedControl
          value={theme}
          onChange={(v) => setTheme(v as ThemeMode)}
          options={[
            { value: 'light', label: '浅色' },
            { value: 'dark', label: '深色' },
          ]}
        />
      </SettingRow>

      <SettingRow label="字号" description="调整界面文字的基础大小。">
        <SegmentedControl
          value={fontSize}
          onChange={(v) => setFontSize(v as FontSize)}
          options={[
            { value: 'small', label: '小' },
            { value: 'medium', label: '中' },
            { value: 'large', label: '大' },
          ]}
        />
      </SettingRow>

      <SettingRow label="显示密度" description="紧凑模式可在一屏内展示更多内容。">
        <SegmentedControl
          value={density}
          onChange={(v) => setDensity(v as Density)}
          options={[
            { value: 'comfortable', label: '舒适' },
            { value: 'compact', label: '紧凑' },
          ]}
        />
      </SettingRow>
    </SettingCard>
  );
}

/* ---------------- 语言 ---------------- */

function LanguageGroup() {
  const [language, setLanguage] = useState('zh-CN');
  const [timezone, setTimezone] = useState('Asia/Shanghai');
  const [dateFormat, setDateFormat] = useState('YYYY-MM-DD');

  return (
    <SettingCard
      title="语言与区域"
      description="设置界面语言、时区与日期格式。"
      onSave={() => savePreferences({ language, timezone, dateFormat })}
    >
      <SettingRow label="界面语言" description="选择界面显示语言。">
        <SelectInput
          value={language}
          onChange={setLanguage}
          options={[
            { value: 'zh-CN', label: '简体中文' },
            { value: 'en-US', label: 'English (US)' },
            { value: 'ja-JP', label: '日本語' },
            { value: 'ko-KR', label: '한국어' },
          ]}
        />
      </SettingRow>

      <SettingRow label="时区" description="影响时间戳与日程的显示。">
        <SelectInput
          value={timezone}
          onChange={setTimezone}
          options={[
            { value: 'Asia/Shanghai', label: '(UTC+08:00) 北京、上海' },
            { value: 'Asia/Tokyo', label: '(UTC+09:00) 东京' },
            { value: 'UTC', label: '(UTC) 协调世界时' },
            { value: 'America/Los_Angeles', label: '(UTC-08:00) 太平洋时间' },
          ]}
        />
      </SettingRow>

      <SettingRow label="日期格式" description="选择日期的显示格式。">
        <SelectInput
          value={dateFormat}
          onChange={setDateFormat}
          options={[
            { value: 'YYYY-MM-DD', label: '2026-07-01' },
            { value: 'DD/MM/YYYY', label: '01/07/2026' },
            { value: 'MM/DD/YYYY', label: '07/01/2026' },
          ]}
        />
      </SettingRow>
    </SettingCard>
  );
}

/* ---------------- 通知 ---------------- */

function NotificationsGroup() {
  const [email, setEmail] = useState(true);
  const [push, setPush] = useState(true);
  const [desktop, setDesktop] = useState(false);

  return (
    <SettingCard
      title="通知"
      description="选择你希望接收的通知渠道。"
      onSave={() => savePreferences({ notifications: { email, push, desktop } })}
    >
      <SettingRow label="邮件通知" description="将重要事件通过邮件发送到你的邮箱。">
        <Toggle checked={email} onChange={setEmail} />
      </SettingRow>
      <SettingRow label="推送通知" description="在移动端接收实时推送。">
        <Toggle checked={push} onChange={setPush} />
      </SettingRow>
      <SettingRow label="桌面通知" description="在桌面端通过系统通知中心提醒。">
        <Toggle checked={desktop} onChange={setDesktop} />
      </SettingRow>
    </SettingCard>
  );
}

/* ---------------- 隐私 ---------------- */

function PrivacyGroup() {
  const [analytics, setAnalytics] = useState(false);
  const [crashReports, setCrashReports] = useState(true);

  return (
    <SettingCard
      title="隐私"
      description="控制你的使用数据是否被收集以帮助改进产品。"
      onSave={() => savePreferences({ privacy: { analytics, crashReports } })}
    >
      <SettingRow
        label="使用分析"
        description="允许收集匿名使用数据,帮助我们改进产品体验。"
      >
        <Toggle checked={analytics} onChange={setAnalytics} />
      </SettingRow>
      <SettingRow
        label="崩溃报告"
        description="在发生崩溃时自动上报诊断信息(opt-in)。"
      >
        <Toggle checked={crashReports} onChange={setCrashReports} />
      </SettingRow>
    </SettingCard>
  );
}

/* ---------------- 通用 UI 基础组件 ---------------- */

/** 设置卡片:包含标题、描述、一组设置项与保存按钮。 */
function SettingCard({
  title,
  description,
  onSave,
  children,
}: {
  title: string;
  description: string;
  onSave: () => void;
  children: ReactNode;
}) {
  return (
    <div className="overflow-hidden rounded-xl border border-gray-200 bg-white dark:border-neutral-800 dark:bg-neutral-900">
      <div className="border-b border-gray-100 px-5 py-4 dark:border-neutral-800">
        <h2 className="text-base font-semibold text-gray-800 dark:text-gray-100">{title}</h2>
        <p className="mt-0.5 text-[13px] text-gray-500 dark:text-gray-400">{description}</p>
      </div>

      <div className="divide-y divide-gray-100 dark:divide-neutral-800">{children}</div>

      <div className="flex justify-end border-t border-gray-100 px-5 py-3 dark:border-neutral-800">
        <button
          type="button"
          onClick={onSave}
          className="rounded-md bg-blue-600 px-4 py-1.5 text-[13px] font-medium text-white shadow-sm transition-colors hover:bg-blue-700 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-1 dark:focus-visible:ring-offset-neutral-900"
        >
          保存
        </button>
      </div>
    </div>
  );
}

/** 单行设置项:左侧标签与说明,右侧控件。 */
function SettingRow({
  label,
  description,
  children,
}: {
  label: string;
  description: string;
  children: ReactNode;
}) {
  return (
    <div className="flex items-center justify-between gap-6 px-5 py-4">
      <div className="min-w-0">
        <div className="text-[13px] font-medium text-gray-800 dark:text-gray-100">{label}</div>
        <div className="mt-0.5 text-[12px] text-gray-500 dark:text-gray-400">{description}</div>
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}

/** 开关组件:带过渡动画。 */
function Toggle({
  checked,
  onChange,
}: {
  checked: boolean;
  onChange: (next: boolean) => void;
}) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      onClick={() => onChange(!checked)}
      className={
        'relative inline-flex h-5 w-9 shrink-0 items-center rounded-full transition-colors duration-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-1 dark:focus-visible:ring-offset-neutral-900 ' +
        (checked ? 'bg-blue-600' : 'bg-gray-300 dark:bg-neutral-600')
      }
    >
      <span
        className={
          'inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow transition-transform duration-200 ' +
          (checked ? 'translate-x-[18px]' : 'translate-x-[3px]')
        }
      />
    </button>
  );
}

/** 分段控件。 */
function SegmentedControl({
  value,
  onChange,
  options,
}: {
  value: string;
  onChange: (next: string) => void;
  options: { value: string; label: string }[];
}) {
  return (
    <div className="inline-flex rounded-md border border-gray-200 bg-gray-50 p-0.5 dark:border-neutral-700 dark:bg-neutral-800">
      {options.map((opt) => (
        <button
          key={opt.value}
          type="button"
          onClick={() => onChange(opt.value)}
          className={
            'rounded-[5px] px-3 py-1 text-[12px] font-medium transition-colors ' +
            (value === opt.value
              ? 'bg-white text-blue-700 shadow-sm dark:bg-neutral-700 dark:text-blue-300'
              : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200')
          }
        >
          {opt.label}
        </button>
      ))}
    </div>
  );
}

/** 下拉选择控件。 */
function SelectInput({
  value,
  onChange,
  options,
}: {
  value: string;
  onChange: (next: string) => void;
  options: { value: string; label: string }[];
}) {
  return (
    <select
      value={value}
      onChange={(e) => onChange(e.target.value)}
      className="min-w-[180px] rounded-md border border-gray-200 bg-white px-3 py-1.5 text-[13px] text-gray-800 transition-colors hover:border-gray-300 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-neutral-700 dark:bg-neutral-800 dark:text-gray-100 dark:hover:border-neutral-600"
    >
      {options.map((opt) => (
        <option key={opt.value} value={opt.value}>
          {opt.label}
        </option>
      ))}
    </select>
  );
}
