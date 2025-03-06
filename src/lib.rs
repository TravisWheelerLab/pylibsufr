use pyo3::prelude::*;
use libsufr::{
    suffix_array::SuffixArray,
    types::{
        SufrBuilderArgs, SequenceFileData, SearchOptions, CountOptions, ExtractOptions, ListOptions, LocateOptions, SuffixSortType
    },
    util::read_sequence_file,
};
use chrono::{DateTime, Local};
use std::{
    path::Path,
    ops::Range,
};

#[pyclass]
pub struct PySequenceFileData {
    sequence_file_data: SequenceFileData
}

#[pymethods]
impl PySequenceFileData {
    fn seq(&self) -> PyResult<Vec<u8>> {
        Ok(self.sequence_file_data.seq.clone())
    }
    
    fn start_positions(&self) -> PyResult<Vec<usize>> {
        Ok(self.sequence_file_data.start_positions.clone())
    }
    
    fn sequence_names(&self) -> PyResult<Vec<String>> {
        Ok(self.sequence_file_data.sequence_names.clone())
    }
}

#[pyfunction]
#[pyo3(signature = (
    input, 
    sequence_delimiter = 37, // = ord('%')
))] 
fn py_read_sequence_file(
    input: String,
    sequence_delimiter: u8,
) -> PyResult<PySequenceFileData> {
    Ok(PySequenceFileData {
        sequence_file_data: read_sequence_file(Path::new(&input), sequence_delimiter).unwrap()
    })
}

#[pyclass]
#[derive(Clone)]
pub struct PySufrBuilderArgs {
    sufr_builder_args: SufrBuilderArgs
}

#[pymethods]
impl PySufrBuilderArgs {
    #[new]
    #[pyo3(signature = (
        text, 
        path, 
        sequence_starts, 
        sequence_names, 
        low_memory = true, 
        max_query_len = None, 
        is_dna = false, 
        allow_ambiguity = false, 
        ignore_softmask = false, 
        num_partitions = 16, 
        seed_mask = None, 
        random_seed = 42,
    ))]
    pub fn new(
        text: Vec<u8>,
        path: Option<String>,
        sequence_starts: Vec<usize>,
        sequence_names: Vec<String>,
        low_memory: bool,
        max_query_len: Option<usize>,
        is_dna: bool,
        allow_ambiguity: bool,
        ignore_softmask: bool,
        num_partitions: usize,
        seed_mask: Option<String>,
        random_seed: u64,
    ) -> PyResult<PySufrBuilderArgs> {
        Ok(PySufrBuilderArgs {
            sufr_builder_args: SufrBuilderArgs {
                text: text,
                path: path,
                low_memory: low_memory,
                max_query_len: max_query_len,
                is_dna: is_dna,
                allow_ambiguity: allow_ambiguity,
                ignore_softmask: ignore_softmask,
                sequence_starts: sequence_starts,
                sequence_names: sequence_names,
                num_partitions: num_partitions,
                seed_mask: seed_mask,
                random_seed: random_seed,
            }
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyCountOptions {
    count_options: CountOptions
}

#[pymethods]
impl PyCountOptions {
    #[new]
    #[pyo3(signature = (
        queries, 
        max_query_len = None, 
        low_memory = false,
    ))]
    pub fn new(
        queries: Vec<String>, 
        max_query_len: Option<usize>,
        low_memory: bool,
    ) -> PyResult<PyCountOptions> {
        Ok(PyCountOptions {
            count_options: CountOptions {
                queries: queries,
                max_query_len: max_query_len,
                low_memory: low_memory,
            }
        })
    }
}

#[pyclass]
pub struct PyCountResult {
    #[pyo3(get)]
    pub query_num: usize,

    #[pyo3(get)]
    pub query: String,

    #[pyo3(get)]
    pub count: usize,
}

#[pyclass]
#[derive(Clone)]
pub struct PyExtractOptions {
    extract_options: ExtractOptions
}

#[pymethods]
impl PyExtractOptions {
    #[new]
    #[pyo3(signature = (
        queries, 
        max_query_len = None, 
        low_memory = false, 
        prefix_len = None, 
        suffix_len = None,
    ))]
    pub fn new(
        queries: Vec<String>, 
        max_query_len: Option<usize>,
        low_memory: bool,
        prefix_len: Option<usize>,
        suffix_len: Option<usize>,
    ) -> PyResult<PyExtractOptions> {
        Ok(PyExtractOptions {
            extract_options: ExtractOptions {
                queries: queries,
                max_query_len: max_query_len,
                low_memory: low_memory,
                prefix_len: prefix_len,
                suffix_len: suffix_len,
            }
        })
    }
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct PyExtractResult {
    #[pyo3(get)]
    pub query_num: usize,

    #[pyo3(get)]
    pub query: String,

    #[pyo3(get)]
    pub sequences: Vec<PyExtractSequence>,
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct PyExtractSequence {
    #[pyo3(get)]
    pub suffix: usize,

    #[pyo3(get)]
    pub rank: usize,

    #[pyo3(get)]
    pub sequence_name: String,

    #[pyo3(get)]
    pub sequence_start: usize,

    #[pyo3(get)]
    pub sequence_range: (usize, usize),

    #[pyo3(get)]
    pub suffix_offset: usize,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyListOptions {
    list_options: ListOptions
}

#[pymethods]
impl PyListOptions {
    #[new]
    #[pyo3(signature = (
        ranks, 
        show_rank = false, 
        show_suffix = false, 
        show_lcp = false, 
        len = None, 
        number = None, 
        output = None,
    ))]
    pub fn new(
        ranks: Vec<usize>,
        show_rank: bool,
        show_suffix: bool,
        show_lcp: bool,
        len: Option<usize>,
        number: Option<usize>,
        output: Option<String>,
    ) -> PyResult<PyListOptions> {
        Ok(PyListOptions {
            list_options: ListOptions {
                ranks: ranks,
                show_rank: show_rank,
                show_suffix: show_suffix,
                show_lcp: show_lcp,
                len: len,
                number: number,
                output: output,
            }
        })
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyLocateOptions {
    locate_options: LocateOptions
}

#[pymethods]
impl PyLocateOptions {
    #[new]
    #[pyo3(signature = (
        queries, 
        max_query_len = None, 
        low_memory = false,
    ))]
    pub fn new(
        queries: Vec<String>,
        max_query_len: Option<usize>,
        low_memory: bool,
    ) -> PyResult<PyLocateOptions> {
        Ok(PyLocateOptions{
            locate_options: LocateOptions {
                queries: queries,
                max_query_len: max_query_len,
                low_memory: low_memory,
            }
        })
    }
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct PyLocateResult {
    #[pyo3(get)]
    pub query_num: usize,

    #[pyo3(get)]
    pub query: String,

    #[pyo3(get)]
    pub positions: Vec<PyLocatePosition>,
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct PyLocatePosition {
    #[pyo3(get)]
    pub suffix: usize,

    #[pyo3(get)]
    pub rank: usize,

    #[pyo3(get)]
    pub sequence_name: String,

    #[pyo3(get)]
    pub sequence_position: usize,
}

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub struct PySufrMetadata {
    #[pyo3(get)]
    pub filename: String,

    #[pyo3(get)]
    pub modified: String,

    #[pyo3(get)]
    pub file_size: usize,

    #[pyo3(get)]
    pub file_version: usize,

    #[pyo3(get)]
    pub is_dna: bool,

    #[pyo3(get)]
    pub allow_ambiguity: bool,

    #[pyo3(get)]
    pub ignore_softmask: bool,

    #[pyo3(get)]
    pub text_len: usize,

    #[pyo3(get)]
    pub len_suffixes: usize,

    #[pyo3(get)]
    pub num_sequences: usize,

    #[pyo3(get)]
    pub sequence_starts: Vec<usize>,

    #[pyo3(get)]
    pub sequence_names: Vec<String>,

    #[pyo3(get)]
    pub sort_type: String,
}

#[pyclass]
pub struct PySuffixArray {
    suffix_array: SuffixArray
}

#[pymethods]
impl PySuffixArray {
    #[new]
    pub fn new(args: PySufrBuilderArgs) -> PyResult<PySuffixArray> {
        let low_memory = args.sufr_builder_args.low_memory;
        let path = SuffixArray::write(args.sufr_builder_args).unwrap();
        Self::read(path, low_memory)
    }
    #[staticmethod]
    #[pyo3(signature = (
        filename,  
        low_memory = false,
    ))]
    pub fn read(filename: String, low_memory: bool) -> PyResult<PySuffixArray> {
        Ok(PySuffixArray {
            suffix_array: SuffixArray::read(&filename, low_memory).unwrap()
        })
    }
    pub fn count(&mut self, args: PyCountOptions) -> PyResult<Vec<PyCountResult>> {
        Ok(self.suffix_array.count(args.count_options)
            .unwrap()
            .iter()
            .map(|count_result| PyCountResult {
                query_num:  count_result.query_num.clone(),
                query:      count_result.query.clone(),
                count:      count_result.count.clone(),
            })
            .collect()
        )
    }
    pub fn extract(&mut self, args: PyExtractOptions) -> PyResult<Vec<PyExtractResult>> {
        Ok(self.suffix_array.extract(args.extract_options)
            .unwrap()
            .iter()
            .map(|extract_result| PyExtractResult {
                query_num:      extract_result.query_num.clone(),
                query:          extract_result.query.clone(),
                sequences:      extract_result.sequences.iter().map(|extract_sequence| PyExtractSequence {
                    suffix:         extract_sequence.suffix.clone(),
                    rank:           extract_sequence.rank.clone(),
                    sequence_name:  extract_sequence.sequence_name.clone(),
                    sequence_start: extract_sequence.sequence_start.clone(),
                    sequence_range: (extract_sequence.sequence_range.start, extract_sequence.sequence_range.end),
                    suffix_offset:  extract_sequence.suffix_offset.clone(),
                }).collect(),
            })
            .collect()
        )
    }
    pub fn list(&mut self, args: PyListOptions) -> PyResult<()> {
        Ok(self.suffix_array.list(args.list_options).unwrap())
    }
    pub fn locate(&mut self, args: PyLocateOptions) -> PyResult<Vec<PyLocateResult>> {
        Ok(self.suffix_array.locate(args.locate_options)
            .unwrap()
            .iter()
            .map(|locate_result| PyLocateResult {
                query_num:  locate_result.query_num.clone(),
                query:      locate_result.query.clone(),
                positions:  locate_result.positions.iter().map(|locate_position| PyLocatePosition {
                    suffix:             locate_position.suffix.clone(),
                    rank:               locate_position.rank.clone(),
                    sequence_name:      locate_position.sequence_name.clone(),
                    sequence_position:  locate_position.sequence_position.clone(),
                }).collect(),
            })
            .collect()
        )
    }
    pub fn metadata(&self) -> PyResult<PySufrMetadata> {
        let metadata = self.suffix_array.metadata().unwrap();
        Ok(PySufrMetadata {
            filename:           metadata.filename.clone(),
            modified:           metadata.modified.to_string(),
            file_size:          metadata.file_size.clone(),
            file_version:       metadata.file_version.clone(),
            is_dna:             metadata.is_dna.clone(),
            allow_ambiguity:    metadata.allow_ambiguity.clone(),
            ignore_softmask:    metadata.ignore_softmask.clone(),
            text_len:           metadata.text_len.clone(),
            len_suffixes:       metadata.len_suffixes.clone(),
            num_sequences:      metadata.num_sequences.clone(),
            sequence_starts:    metadata.sequence_starts.clone(),
            sequence_names:     metadata.sequence_names.clone(),
            sort_type:          format!("{:?}", metadata.sort_type),
        })
    }
    #[pyo3(signature = (
        pos,
        len = None,
    ))]
    pub fn string_at(&mut self, pos: usize, len: Option<usize>) -> PyResult<String> {
        Ok(self.suffix_array.string_at(pos, len).unwrap())
    }
}

#[pymodule]
fn pylibsufr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_read_sequence_file, m)?)?;
    m.add_class::<PyCountResult>()?;
    m.add_class::<PyCountOptions>()?;
    m.add_class::<PyExtractResult>()?;
    m.add_class::<PyExtractSequence>()?;
    m.add_class::<PyExtractOptions>()?;
    m.add_class::<PyListOptions>()?;
    m.add_class::<PyLocateResult>()?;
    m.add_class::<PyLocatePosition>()?;
    m.add_class::<PyLocateOptions>()?;
    m.add_class::<PySufrMetadata>()?;
    m.add_class::<PySufrBuilderArgs>()?;
    m.add_class::<PySuffixArray>()?;
    Ok(())
}
