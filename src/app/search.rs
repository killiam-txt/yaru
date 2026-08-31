use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::app::{App, Screen};

impl App {
    pub fn search(&mut self) {
        if self.input.is_empty() {
            return;
        }
        self.selected = 0;
        self.result_ids = Vec::new();
        self.detail = None;
        self.insert_mode = false;

        Python::with_gil(|py| {
            let result: PyResult<()> = (|| {
                let module = PyModule::import(py, "yaru_core.api")?;
                let search_fn = module.getattr("search_anime")?;
                let response = search_fn.call1((&self.input,))?;

                let media = response
                    .downcast::<PyDict>()?
                    .get_item("media")?
                    .unwrap();

                let media_list = media.downcast::<pyo3::types::PyList>()?;

                let mut ids = Vec::new();
                self.results = media_list
                    .iter()
                    .map(|item| {
                        let item = item.downcast::<PyDict>().unwrap();

                        let id: i64 = item.get_item("id").unwrap()
                            .map(|v| v.extract::<i64>().unwrap_or(0))
                            .unwrap_or(0);
                        ids.push(id);

                        let binding = item.get_item("title").unwrap().unwrap();
                        let title_obj = binding.downcast::<PyDict>().unwrap();

                        let english = title_obj.get_item("english").unwrap();
                        let romaji = title_obj.get_item("romaji").unwrap();

                        let title = match english {
                            Some(e) if !e.is_none() => e.to_string(),
                            _ => romaji.map(|r| r.to_string()).unwrap_or_default(),
                        };

                        let year = item.get_item("seasonYear").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                        let score = item.get_item("averageScore").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                        let eps = item.get_item("episodes").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                        let status = item.get_item("status").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());

                        format!(
                            "{:<35} [{:>4}] | score: {:>4} | eps: {:>4} | {}",
                            title, year, score, eps, status
                        )
                    })
                    .collect();
                self.result_ids = ids;

                self.error = None;
                Ok(())
            })();

            if let Err(e) = result {
                self.error = Some(format!("[error] {}", e));
            }
        });
    }

    pub(crate) fn fetch_detail_by_id(&mut self, id: i64) {
        Python::with_gil(|py| {
            let result: PyResult<()> = (|| {
                let module = PyModule::import(py, "yaru_core.api")?;
                let detail_fn = module.getattr("get_anime_details")?;
                let response = detail_fn.call1((id,))?;
                let media = response.downcast::<PyDict>()?;

                let binding = media.get_item("title").unwrap().unwrap();
                let title_obj = binding.downcast::<PyDict>().unwrap();
                let english = title_obj.get_item("english").unwrap();
                let romaji = title_obj.get_item("romaji").unwrap();
                let title = match english {
                    Some(e) if !e.is_none() => e.to_string(),
                    _ => romaji.map(|r| r.to_string()).unwrap_or_default(),
                };

                let year = media.get_item("seasonYear").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                let score = media.get_item("averageScore").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                let eps = media.get_item("episodes").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                let status = media.get_item("status").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                let genres = media.get_item("genres").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                let desc = media.get_item("description").unwrap()
                    .map(|v| v.to_string()).unwrap_or("N/A".to_string());

                let genres_clean = genres
                    .trim_matches(|c| c == '[' || c == ']')
                    .replace('\'', "")
                    .split(", ")
                    .collect::<Vec<&str>>()
                    .join(" - ");

                let desc_clean = desc
                    .replace("<br>", "\n")
                    .replace("<br />", "\n")
                    .replace("&amp;", "&")
                    .replace("&lt;", "<")
                    .replace("&gt;", ">");

                self.detail = Some(format!(
                    "{} [{}]\n\ngenres: {}\n\n{}\n\nscore:  {}\neps:    {}\nstatus: {}",
                    title, year, genres_clean, desc_clean.trim(), score, eps, status
                ));
                Ok(())
            })();

            if let Err(e) = result {
                self.detail = Some(format!("[error] {}", e));
            }
        });

        self.previous_screen = self.screen.clone();
        self.screen = Screen::Detail;
    }

    pub fn fetch_detail(&mut self) {
        if self.result_ids.is_empty() {
            return;
        }
        let id = self.result_ids[self.selected];
        self.fetch_detail_by_id(id);
    }

    pub fn fetch_detail_from_list(&mut self) {
        if self.list_ids.is_empty() {
            return;
        }
        let id = self.list_ids[self.list_selected];
        self.fetch_detail_by_id(id);
    }
}