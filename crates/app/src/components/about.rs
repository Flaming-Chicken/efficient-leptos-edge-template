//! About & Architecture view page component.

use crate::storage::StorageDiagnostics;
use leptos::prelude::*;
use shared::AppState;

/// Architecture overview, live storage diagnostics, and keyboard shortcuts page.
#[component]
pub fn AboutPage(
    /// Root application state signal.
    state: RwSignal<AppState>,
    /// Live storage diagnostics memo.
    diag: Memo<StorageDiagnostics>,
) -> impl IntoView {
    view! {
        <div
            class="about-page-container"
            style="max-width: 900px; margin: 0 auto; padding: 24px 16px;"
        >
            // Hero / Title Section
            <div class="about-hero" style="text-align: center; margin-bottom: 32px;">
                <h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 8px;">
                    "Leptos Serverless Edge Template"
                </h1>
                <p style="color: var(--text-muted); font-size: 1.1rem; max-width: 650px; margin: 0 auto 12px;">
                    "A pure zero-backend static SSR + WASM hydration foundation engineered exclusively for global edge networks"
                </p>
                <span
                    class="badge"
                    style="background: var(--bg-surface-elevated); padding: 4px 12px; border-radius: 9999px; font-family: var(--font-mono); font-size: 0.85rem; border: 1px solid var(--border-color);"
                >
                    {concat!(
                        "Version ",
                        env!("CARGO_PKG_VERSION"),
                        " • Rust 2024 • Leptos 0.8 • Static SSR + WASM Hydration",
                    )}
                </span>
            </div>

            // Card 1: Four-Crate Pure Serverless Edge Architecture
            <div
                class="card"
                style="background: var(--bg-surface); border: 1px solid var(--border-color); border-radius: 12px; padding: 24px; margin-bottom: 24px; box-shadow: var(--shadow-sm);"
            >
                <h2 style="font-size: 1.3rem; font-weight: 600; margin-bottom: 12px; display: flex; align-items: center; gap: 8px;">
                    "🏛️ Four-Crate Pure Serverless Edge Architecture"
                </h2>
                <p style="color: var(--text-muted); margin-bottom: 16px;">
                    "Designed exclusively for serverless edge deployments (zero desktop or server-runtime dependencies):"
                </p>

                <div class="arch-list" style="display: flex; flex-direction: column; gap: 12px;">
                    <div style="display: flex; gap: 16px; align-items: baseline; padding: 8px 12px; background: var(--bg-surface-elevated); border-radius: 8px;">
                        <code style="font-weight: 700; color: var(--accent); min-width: 140px;">
                            "crates/shared"
                        </code>
                        <span style="color: var(--text-primary);">
                            "Domain entities, AppState, spodeian-parallel Rayon/WASM sweeps, WGSL compute shader, and JSON/CSV/BSON/SVG/HTML export engines."
                        </span>
                    </div>
                    <div style="display: flex; gap: 16px; align-items: baseline; padding: 8px 12px; background: var(--bg-surface-elevated); border-radius: 8px;">
                        <code style="font-weight: 700; color: var(--accent); min-width: 140px;">
                            "crates/app"
                        </code>
                        <span style="color: var(--text-primary);">
                            "Leptos reactive UI components, SEO shell (pre-paint theme/dyslexia + JSON-LD), Command Palette, Analytics SVG Explorer, and IndexedDB storage."
                        </span>
                    </div>
                    <div style="display: flex; gap: 16px; align-items: baseline; padding: 8px 12px; background: var(--bg-surface-elevated); border-radius: 8px;">
                        <code style="font-weight: 700; color: var(--accent); min-width: 140px;">
                            "crates/web"
                        </code>
                        <span style="color: var(--text-primary);">
                            "Pure WASM hydration entrypoint (hydrate feature only), PWA Service Worker, offline fonts, Cloudflare _headers, and _redirects."
                        </span>
                    </div>
                    <div style="display: flex; gap: 16px; align-items: baseline; padding: 8px 12px; background: var(--bg-surface-elevated); border-radius: 8px;">
                        <code style="font-weight: 700; color: var(--accent); min-width: 140px;">
                            "crates/ssg"
                        </code>
                        <span style="color: var(--text-primary);">
                            "Build-time static site generator pre-rendering routes to HTML, computing SHA-256 CSP hashes, sitemap.xml, robots.txt, and sw.js manifests."
                        </span>
                    </div>
                </div>
            </div>

            // Card 2: Live Diagnostics & Storage
            <div
                class="card"
                style="background: var(--bg-surface); border: 1px solid var(--border-color); border-radius: 12px; padding: 24px; margin-bottom: 24px; box-shadow: var(--shadow-sm);"
            >
                <h2 style="font-size: 1.3rem; font-weight: 600; margin-bottom: 12px; display: flex; align-items: center; gap: 8px;">
                    "📊 Live System Diagnostics & Storage"
                </h2>

                <div
                    class="diag-grid"
                    style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px;"
                >
                    <div style="padding: 12px; background: var(--bg-surface-elevated); border-radius: 8px;">
                        <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase;">
                            "Storage Tier"
                        </div>
                        <div style="font-weight: 600; margin-top: 4px;">
                            {move || diag.get().backend.label()}
                        </div>
                    </div>
                    <div style="padding: 12px; background: var(--bg-surface-elevated); border-radius: 8px;">
                        <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase;">
                            "Persistence Mode"
                        </div>
                        <div style="font-weight: 600; margin-top: 4px;">
                            {move || match diag.get().is_persisted {
                                Some(true) => "Persistent (Guaranteed)",
                                Some(false) => "Ephemeral (Browser Managed)",
                                None => "Native / Unknown",
                            }}
                        </div>
                    </div>
                    <div style="padding: 12px; background: var(--bg-surface-elevated); border-radius: 8px;">
                        <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase;">
                            "Memory Tier (RAM)"
                        </div>
                        <div style="font-weight: 600; margin-top: 4px;">
                            {move || {
                                format!(
                                    "{} items (Limit: {} items)",
                                    diag.get().memory_items_count,
                                    diag.get().memory_capacity_limit,
                                )
                            }}
                        </div>
                    </div>
                    <div style="padding: 12px; background: var(--bg-surface-elevated); border-radius: 8px;">
                        <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase;">
                            "IndexedDB Fallback"
                        </div>
                        <div style="font-weight: 600; margin-top: 4px; color: #10b981;">
                            {move || {
                                if diag.get().idb_active || diag.get().fallback_tier_active {
                                    "Active (Protected against RAM overfill)"
                                } else {
                                    "Synchronized & Available"
                                }
                            }}
                        </div>
                    </div>
                    <div style="padding: 12px; background: var(--bg-surface-elevated); border-radius: 8px;">
                        <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase;">
                            "Total Managed Tasks"
                        </div>
                        <div style="font-weight: 600; margin-top: 4px;">
                            {move || {
                                format!(
                                    "{} items ({} completed)",
                                    state.get().collection.total_count(),
                                    state.get().collection.completed_count(),
                                )
                            }}
                        </div>
                    </div>
                    <div style="padding: 12px; background: var(--bg-surface-elevated); border-radius: 8px;">
                        <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase;">
                            "Active Theme"
                        </div>
                        <div style="font-weight: 600; margin-top: 4px;">
                            {move || {
                                format!(
                                    "{} {}",
                                    state.get().config.theme.icon(),
                                    state.get().config.theme.label(),
                                )
                            }}
                        </div>
                    </div>
                </div>
            </div>

            // Card 3: Shortcuts & Guidance
            <div
                class="card"
                style="background: var(--bg-surface); border: 1px solid var(--border-color); border-radius: 12px; padding: 24px; box-shadow: var(--shadow-sm);"
            >
                <h2 style="font-size: 1.3rem; font-weight: 600; margin-bottom: 12px; display: flex; align-items: center; gap: 8px;">
                    "⌨️ Keyboard Shortcuts & Features"
                </h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 12px;">
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <kbd style="padding: 4px 8px; background: var(--bg-surface-elevated); border: 1px solid var(--border-color); border-radius: 4px; font-family: var(--font-mono); font-size: 0.85rem;">
                            "Enter"
                        </kbd>
                        <span>"Submit new item from the input form"</span>
                    </div>
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <kbd style="padding: 4px 8px; background: var(--bg-surface-elevated); border: 1px solid var(--border-color); border-radius: 4px; font-family: var(--font-mono); font-size: 0.85rem;">
                            "Escape"
                        </kbd>
                        <span>"Close active modal dialogs"</span>
                    </div>
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <kbd style="padding: 4px 8px; background: var(--bg-surface-elevated); border: 1px solid var(--border-color); border-radius: 4px; font-family: var(--font-mono); font-size: 0.85rem;">
                            "Ctrl+K / Cmd+K"
                        </kbd>
                        <span>"Open searchable Command Palette"</span>
                    </div>
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <kbd style="padding: 4px 8px; background: var(--bg-surface-elevated); border: 1px solid var(--border-color); border-radius: 4px; font-family: var(--font-mono); font-size: 0.85rem;">
                            "Ctrl+Z / Ctrl+Y"
                        </kbd>
                        <span>"Undo or redo item collection edits"</span>
                    </div>
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <span style="font-weight: 600;">"Data Export:"</span>
                        <span>
                            "JSON, CSV, Compressed BSON, SVG Chart, Printable HTML & #state Links"
                        </span>
                    </div>
                    <div style="display: flex; gap: 12px; align-items: center;">
                        <span style="font-weight: 600;">"Offline PWA:"</span>
                        <span>"Local icons, service worker cache-first"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
