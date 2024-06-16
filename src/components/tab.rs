use super::{Component, DrawableComponent, EventState};
use crate::components::command::{self, CommandInfo};
use crate::config::KeyConfig;
use crate::event::Key;
use anyhow::Result;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs},
    Frame,
};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Tab {
    Records,
    Properties,
    Sql,
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct TabComponent {
    pub selected_tab: Tab,
    key_config: KeyConfig,
}

impl TabComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        Self {
            selected_tab: Tab::Records,
            key_config,
        }
    }

    pub fn reset(&mut self) {
        self.selected_tab = Tab::Records;
    }

    fn names(&self) -> Vec<String> {
        vec![
            command::tab_records(&self.key_config).name,
            command::tab_properties(&self.key_config).name,
            command::tab_sql_editor(&self.key_config).name,
        ]
    }
}

impl DrawableComponent for TabComponent {
    fn draw(&self, f: &mut Frame, area: Rect, _focused: bool) -> Result<()> {
        let titles: Vec<_> = self.names().iter().cloned().map(Line::from).collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .select(self.selected_tab as usize)
            .style(Style::default().fg(Color::DarkGray))
            .highlight_style(
                Style::default()
                    .fg(Color::Reset)
                    .add_modifier(Modifier::UNDERLINED),
            );
        f.render_widget(tabs, area);
        Ok(())
    }
}

impl Component for TabComponent {
    fn commands(&self, _out: &mut Vec<CommandInfo>) {}

    fn event(&mut self, key: Key) -> Result<EventState> {
        if key == self.key_config.tab_records {
            self.selected_tab = Tab::Records;
            return Ok(EventState::Consumed);
        } else if key == self.key_config.tab_sql_editor {
            self.selected_tab = Tab::Sql;
            return Ok(EventState::Consumed);
        } else if key == self.key_config.tab_properties {
            self.selected_tab = Tab::Properties;
            return Ok(EventState::Consumed);
        }
        Ok(EventState::NotConsumed)
    }
}
