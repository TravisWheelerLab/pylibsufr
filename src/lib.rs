use pyo3::prelude::*;
use libsufr::{
    suffix_array::SuffixArray,
    types::{
        SufrBuilderArgs, SequenceFileData, CountOptions, CountResult
    },
    util::read_sequence_file,
};
use std::path::Path;

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
#[pyo3(signature = (input, sequence_delimiter))]
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
    #[pyo3(signature = (text, path, low_memory, max_query_len, is_dna, allow_ambiguity, ignore_softmask, sequence_starts, sequence_names, num_partitions, seed_mask, random_seed))]
    pub fn new(
        text: Vec<u8>,
        path: Option<String>,
        low_memory: bool,
        max_query_len: Option<usize>,
        is_dna: bool,
        allow_ambiguity: bool,
        ignore_softmask: bool,
        sequence_starts: Vec<usize>,
        sequence_names: Vec<String>,
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
pub struct PyCountOptions {
    count_options: CountOptions
}

impl Clone for PyCountOptions {
    fn clone(&self) -> Self {
        PyCountOptions {
            count_options: CountOptions {
                queries: self.count_options.queries.clone(),
                max_query_len: self.count_options.max_query_len.clone(),
                low_memory: self.count_options.low_memory.clone(),
            }
        }
    }
}

#[pymethods]
impl PyCountOptions {
    #[new]
    #[pyo3(signature = (queries, max_query_len, low_memory))]
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
    /// The ordinal position of the original query
    #[pyo3(get)]
    pub query_num: usize,

    /// The query string
    #[pyo3(get)]
    pub query: String,

    /// Number of times a query was found
    #[pyo3(get)]
    pub count: usize,
}

#[pyclass]
pub struct PySuffixArray {
    suffix_array: SuffixArray
}

unsafe impl Send for PySuffixArray {}
unsafe impl Sync for PySuffixArray {}



#[pymethods]
impl PySuffixArray {
    #[new]
    pub fn new(args: PySufrBuilderArgs) -> PyResult<PySuffixArray> {
        let low_memory = args.sufr_builder_args.low_memory;
        let path = SuffixArray::write(args.sufr_builder_args).unwrap();
        Self::read(path, low_memory)
        //Ok(PySuffixArray {
        //    suffix_array: SuffixArray::read(&path, low_memory).unwrap()
        //})
    }
    #[staticmethod]
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
                query_num: count_result.query_num.clone(),
                query: count_result.query.clone(),
                count: count_result.count.clone(),
            })
            .collect()
        )
    }
    // pub fn extract(&mut self, args: PyExtractOptions) -> PyResult<Vec<PyExtractResult>> {
    //     Ok(self.suffix_array.extract(args.extract_options).unwrap())
    // }
    // pub fn metadata(&self) -> Result<PySufrMetadata> {
    //     Ok(self.suffix_array.metadata().unwrap())
    // }
    // pub fn list(&mut self, args: PyListOptions) -> PyResult<()> {
    //     Ok(self.suffix_array.list(args.list_options).unwrap())
    // }
    // pub fn locate(&mut self, args: PyLocateOptions) -> PyResult<Vec<PyLocateResult>> {
    //     Ok(self.suffix_array.locate(args.locate_options).unwrap())
    // }
    // pub fn string_at(&mut self, pos: usize, len: Option<usize>) -> PyResult<String> {
    //     Ok(self.suffix_array.string_at(pos, len).unwrap())
    // }
}


#[pymodule]
fn _pylibsufr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_read_sequence_file, m)?)?;
    m.add_class::<PyCountResult>()?;
    m.add_class::<PyCountOptions>()?;
    m.add_class::<PySufrBuilderArgs>()?;
    m.add_class::<PySuffixArray>()?;
    Ok(())
}
//use anyhow::Result;
//use libsufr::{
//    suffix_array::SuffixArray,
//    types::{
//        CountOptions, ExtractOptions, ListOptions, LocateOptions, SuffixSortType,
//        SufrBuilderArgs,
//    },
//    util::read_sequence_file,
//};
//use regex::Regex;
//use std::{
//    ffi::OsStr,
//    fmt::Debug,
//    fs::{self, File},
//    io::{self, Write},
//    ops::Range,
//    path::{Path, PathBuf},
//    time::Instant,
//    iter::zip,
//    sync::Mutex,
//};
//
//fn parse_locate_queries(queries: &[String]) -> Result<Vec<String>> {
//    let whitespace = Regex::new(r"\s+").unwrap();
//    let mut ret = vec![];
//    for query in queries {
//        if Path::new(&query).exists() {
//            let contents = fs::read_to_string(query)?;
//            let mut vals: Vec<String> = whitespace
//                .split(&contents)
//                .filter(|v| !v.is_empty())
//                .map(|v| v.to_string())
//                .collect();
//            ret.append(&mut vals);
//        } else {
//            ret.push(query.to_string());
//        }
//    }
//
//    Ok(ret)
//}


// 
// #[pyclass]
// pub struct PyExtractOptions {
//     extract_options: ExtractOptions
// }
// 
// #[pyclass]
// pub struct PyListOptions {
//     list_options: ListOptions
// }
// 
// #[pyclass]
// pub struct PyLocateOptions {
//     locate_options: LocateOptions
// }
// 
// #[pyclass]
// pub struct PyExtractResult {
//     extract_result: ExtractResult
// }
// 
// #[pyclass]
// pub struct PySufrMetadata {
//     sufr_metadata: SufrMetadata
// }
// 
// #[pyclass]
// pub struct PyLocateResult {
//     locate_result: LocateResult
// }