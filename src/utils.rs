use crate::env;
use std::iter;
// use web_sys::Storage;

pub fn make_backend_url(relative: &str) -> String {
    format!("{}{}", env::APP_BACKEND_URL, relative)
}

// #[inline]
// pub fn local_storage() -> Storage {
//     web_sys::window()
//         .expect("Can't access to the window")
//         .local_storage()
//         .expect("Can't access to local storage")
//         .expect("Can't access to local storage")
// }

pub struct Paginator {
    pages: usize,
    page: usize,
}

impl Paginator {
    pub fn new(page: usize, pages: usize) -> Self {
        Self { pages, page }
    }

    pub fn iter_pages(
        &self,
        left_edge: usize,
        left_current: usize,
        right_current: usize,
        right_edge: usize,
    ) -> impl Iterator<Item = Option<usize>> {
        let pages_end = self.pages + 1;

        if pages_end == 1 {
            return Box::new(iter::empty()) as Box<dyn Iterator<Item = Option<usize>>>;
        }

        let left_end = (1 + left_edge).min(pages_end);
        let mut result = Vec::new();

        result.extend((1..left_end).map(Some));

        if left_end == pages_end {
            return Box::new(result.into_iter());
        }

        let mid_start = left_end.max(self.page.saturating_sub(left_current));
        let mid_end = (self.page + right_current + 1).min(pages_end);

        if mid_start > left_end {
            result.push(None);
        }

        result.extend((mid_start..mid_end).map(Some));

        if mid_end == pages_end {
            return Box::new(result.into_iter());
        }

        let right_start = mid_end.max(pages_end.saturating_sub(right_edge));

        if right_start > mid_end {
            result.push(None);
        }

        result.extend((right_start..pages_end).map(Some));

        Box::new(result.into_iter())
    }
}
