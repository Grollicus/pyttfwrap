use std::sync::Arc;

use pyo3::{create_exception, exceptions::{PyException, PyValueError}, prelude::*};
use owned_ttf_parser::{OwnedFace, AsFaceRef};
use ttf_word_wrap::{CharPosition, TTFParserMeasure, WhiteSpaceWordWrap, Wrap, WrapWithPosition};
use unicode_segmentation::UnicodeSegmentation;


create_exception!(pyttfwrap, PyttfWrapError, PyException);


/// Class to calculate how text will wrap using a given TrueType font and a given available width
#[pyclass]
struct TextWrapper {
    inner: Arc<_TextWrapper>
}

struct _TextWrapper {
    font_path: String,
    base_character: String,
    font: OwnedFace,
    base_size: f64,
}

impl _TextWrapper {
    fn new(file_path: &str, base_character: &str) -> PyResult<Self> {
        let font_data: Vec<u8> = std::fs::read(file_path).map_err(|e| PyttfWrapError::new_err(format!("loading ttf failed: {}", e)))?;
        let font = OwnedFace::from_vec(font_data, 0).map_err(|e| PyttfWrapError::new_err(format!("parsing font failed: {}", e)))?;
        let base_measurement = {
            let measure = TTFParserMeasure::new(font.as_face_ref());
            match base_character.wrap_with_position(&WhiteSpaceWordWrap::new(999999, &measure)).next() {
                Some(CharPosition::Known(p)) => p.width as f64,
                _ => return Err(PyttfWrapError::new_err(format!("Given font does not contain '{}'", base_character))),
            }
        };

        Ok(Self {
            font_path: file_path.to_string(),
            base_character: base_character.to_string(),
            font,
            base_size: base_measurement,
        })
    }

    fn wrap(&self, m_characters_in_a_line: f64, text: &str) -> Vec<String> {
        let measure = TTFParserMeasure::new(self.font.as_face_ref());
        let max_width = (m_characters_in_a_line * self.base_size).floor() as u32;
        let word_wrapper = WhiteSpaceWordWrap::new(max_width, &measure);
        return text.wrap(&word_wrapper).map(|s| s.to_string()).collect()
    }
}

#[pymethods]
impl TextWrapper {
    /// Creates a new instance of TextWrapper.
    #[new]
    #[pyo3(signature = (file_path, base_character = "0"))]
    fn new(file_path: &str, base_character: &str) -> PyResult<Self> {
        if base_character.graphemes(true).count() != 1 {
            return Err(PyValueError::new_err("base_character needs to be exactly 1 character!"))
        }
        Ok(Self {
            inner: Arc::new(_TextWrapper::new(file_path, base_character)?)
        })
    }

    /// Returns a list of lines that will result when wrapping `text` using the given font and width. Width is computed to be `line_width` times the width of the `base_character`.
    #[pyo3(signature = (line_width, text))]
    fn wrap(&self, line_width: f64, text: &str) -> Vec<String> {
        self.inner.wrap(line_width, text)
    }

    fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("TextWrapper font_path={:?} base_character={:?}", self.inner.font_path, self.inner.base_character))
    }
}

/// Computes text wraps based on a font and given width.
#[pymodule]
fn pyttfwrap(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TextWrapper>()?;
    m.add("PyttfWrapError", py.get_type::<PyttfWrapError>())?;

    Ok(())
}
