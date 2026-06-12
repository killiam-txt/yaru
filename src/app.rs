use pyo3::prelude::*;
use pyo3::types::PyDict;

pub enum Screen {
    Search,
    Detail,
    List,
}

pub struct App {
    pub input: String,
    pub results: Vec<String>,
    pub result_ids: Vec<i64>,
    pub error: Option<String>,
    pub selected: usize,
    pub detail: Option<String>,
    pub screen: Screen,
    pub list_items: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            results: Vec::new(),
            result_ids: Vec::new(),
            error: None,
            selected: 0,
            detail: None,
            screen: Screen::Search,
            list_items: Vec::new(),
        }
    }

    pub fn search(&mut self) {
        if self.input.is_empty() {
            return;
        }
        self.selected = 0;
        self.result_ids = Vec::new();
        self.detail = None;

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

    pub fn fetch_detail(&mut self) {
        if self.result_ids.is_empty() {
            return;
        }
        let id = self.result_ids[self.selected];

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

                self.detail = Some(format!(
                    "{} [{}]\n\nscore: {} | eps: {} | {}\ngenres: {}\n\n{}",
                    title, year, score, eps, status, genres, desc
                ));
                Ok(())
            })();

            if let Err(e) = result {
                self.detail = Some(format!("[error] {}", e));
            }
        });

        self.screen = Screen::Detail;
    }

    pub fn add_to_list(&mut self) {
        if self.result_ids.is_empty() {
            return;
        }
        let id = self.result_ids[self.selected];

        Python::with_gil(|py| {
            let result: PyResult<()> = (|| {
                let api_module = PyModule::import(py, "yaru_core.api")?;
                let detail_fn = api_module.getattr("get_anime_details")?;
                let anime = detail_fn.call1((id,))?;

                let db_module = PyModule::import(py, "yaru_core.db")?;
                let add_fn = db_module.getattr("add_anime")?;
                add_fn.call1((anime,))?;
                Ok(())
            })();

            if let Err(e) = result {
                self.error = Some(format!("[error] {}", e));
            }
        });
    }

    pub fn fetch_list(&mut self) {
        Python::with_gil(|py| {
            let result: PyResult<()> = (|| {
                let module = PyModule::import(py, "yaru_core.db")?;
                let get_fn = module.getattr("get_list")?;
                let response = get_fn.call0()?;
                let items = response.downcast::<pyo3::types::PyList>()?;

                self.list_items = items
                    .iter()
                    .map(|item| {
                        let item = item.downcast::<PyDict>().unwrap();
                        let title = item.get_item("title").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                        let year = item.get_item("year").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                        let score = item.get_item("score").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                        let eps = item.get_item("episodes").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());
                        let status = item.get_item("watch_status").unwrap()
                            .map(|v| v.to_string()).unwrap_or("N/A".to_string());

                        format!(
                            "{:<35} [{:>4}] | score: {:>4} | eps: {:>4} | {}",
                            title, year, score, eps, status
                        )
                    })
                    .collect();

                Ok(())
            })();

            if let Err(e) = result {
                self.error = Some(format!("[error] {}", e));
            }
        });

        self.screen = Screen::List;
    }
}