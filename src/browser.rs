use crate::color_parser;

pub fn main() -> String {
    let colors = color_parser::main();
    let css = format!(
        r#"
@media (prefers-color-scheme: dark) {{
  :root {{
    --zen-colors-primary: {0} !important;
    --zen-primary-color: {1} !important;
    --zen-colors-secondary: {0} !important;
    --zen-colors-tertiary: {2} !important;
    --zen-colors-border: {1} !important;
    --toolbarbutton-icon-fill: {1} !important;
    --lwt-text-color: {3} !important;
    --toolbar-field-color: {3} !important;
    --tab-selected-textcolor: rgb(171, 212, 223) !important;
    --toolbar-field-focus-color: {3} !important;
    --toolbar-color: {3} !important;
    --newtab-text-primary-color: {3} !important;
    --arrowpanel-color: {3} !important;
    --arrowpanel-background: {4} !important;
    --sidebar-text-color: {3} !important;
    --lwt-sidebar-text-color: {3} !important;
    --lwt-sidebar-background-color: {5} !important;
    --toolbar-bgcolor: {0} !important;
    --newtab-background-color: {4} !important;
    --zen-themed-toolbar-bg: {2} !important;
    --zen-main-browser-background: {2} !important;
    --toolbox-bgcolor-inactive: {2} !important;
  }}

  #permissions-granted-icon {{
    color: {2} !important;
  }}

  .sidebar-placesTree {{
    background-color: {4} !important;
  }}

  #zen-workspaces-button {{
    background-color: {4} !important;
  }}

  #TabsToolbar {{
    background-color: {2} !important;
  }}

  .urlbar-background {{
    background-color: {4} !important;
  }}

  .content-shortcuts {{
    background-color: {4} !important;
    border-color: {1} !important;
  }}

  .urlbarView-url {{
    color: {1} !important;
  }}

  #zenEditBookmarkPanelFaviconContainer {{
    background: {5} !important;
  }}

  #zen-media-controls-toolbar {{
    & #zen-media-progress-bar {{
      &::-moz-range-track {{
        background: {0} !important;
      }}
    }}
  }}

  toolbar .toolbarbutton-1 {{
    &:not([disabled]) {{
      &:is([open], [checked])
        > :is(
          .toolbarbutton-icon,
          .toolbarbutton-text,
          .toolbarbutton-badge-stack
        ) {{
        fill: {5};
      }}
    }}
  }}

  .identity-color-blue {{
    --identity-tab-color: #8aadf4 !important;
    --identity-icon-color: #8aadf4 !important;
  }}

  .identity-color-turquoise {{
    --identity-tab-color: #8bd5ca !important;
    --identity-icon-color: #8bd5ca !important;
  }}

  .identity-color-green {{
    --identity-tab-color: #a6da95 !important;
    --identity-icon-color: #a6da95 !important;
  }}

  .identity-color-yellow {{
    --identity-tab-color: #eed49f !important;
    --identity-icon-color: #eed49f !important;
  }}

  .identity-color-orange {{
    --identity-tab-color: #f5a97f !important;
    --identity-icon-color: #f5a97f !important;
  }}

  .identity-color-red {{
    --identity-tab-color: #ed8796 !important;
    --identity-icon-color: #ed8796 !important;
  }}

  .identity-color-pink {{
    --identity-tab-color: #f5bde6 !important;
    --identity-icon-color: #f5bde6 !important;
  }}

  .identity-color-purple {{
    --identity-tab-color: #c6a0f6 !important;
    --identity-icon-color: #c6a0f6 !important;
  }}

  hbox#titlebar {{
    background-color: {2} !important;
  }}

  #zen-appcontent-navbar-container {{
    background-color: {2} !important;
  }}
}}
"#,
        colors[0], colors[1], colors[6], colors[5], colors[7], colors[8]
    );
    css
}
