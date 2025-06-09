use crate::app::App;

impl App<'_> {
    pub fn update_discussion_scrollbar(&mut self) {
        let line_count = self.get_messages_lines_count() as u16;
        let max_scroll = line_count.saturating_sub(self.last_messages_area_size.1);

        if self.last_messages_area_size.1 > 0 {
            self.discussion_scrollbar.set_max_scroll(max_scroll);
            self.discussion_scrollbar.bottom();
        }
        else {
            self.discussion_scrollbar.set_max_scroll(0);
            self.discussion_scrollbar.scroll = 0;
        }
    }
}