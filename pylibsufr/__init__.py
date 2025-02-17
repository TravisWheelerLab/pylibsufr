import _pylibsufr

def test_create_and_count(input_file: str, output_file: str, queries: list[str]):
    sequence_file_data = _pylibsufr.py_read_sequence_file(
        input_file, 
        ord('%'))
    sufr_builder_args = _pylibsufr.PySufrBuilderArgs(
        sequence_file_data.seq(), 
        output_file, 
        False, 
        None, 
        True, 
        False, 
        False, 
        sequence_file_data.start_positions(), 
        sequence_file_data.sequence_names(), 
        16, 
        None, 
        42)
    suffix_array = _pylibsufr.PySuffixArray(sufr_builder_args)
    count_options = _pylibsufr.PyCountOptions(
        queries,
        None,
        False
    )
    count_results = suffix_array.count(count_options)
    for res in count_results:
        print(res.query_num, res.query, res.count)