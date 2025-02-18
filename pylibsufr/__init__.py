import _pylibsufr
from sys import stdout

TEST_INPUT = "test.fa"
TEST_OUTPUT = "_test.sufr"
TEST_QUERIES = ["TT", "GATT", "ATTA", "ACTG"]
TEST_LIST_OUTPUT = "test.list"
EXPECTED_COUNT_DATA = [3,2,3,2]
EXPECTED_EXTRACT_DATA = [
    [(27, 27, '4', 25, (2, 33), 0), (2, 28, '1', 0, (2, 8), 0), (11, 29, '2', 8, (3, 16), 0)],
    [(25, 20, '4', 25, (0, 33), 0), (0, 21, '1', 0, (0, 8), 0)],
    [(26, 9, '4', 25, (1, 33), 0), (1, 10, '1', 0, (1, 8), 0), (10, 11, '2', 8, (2, 16), 0)],
    [(20, 6, '3', 16, (4, 25), 0), (16, 7, '3', 16, (0, 25), 0)],
]
EXPECTED_LOCATE_DATA = [
    [(27, 27, '4', 2), (2, 28, '1', 2), (11, 29, '2', 3)], 
    [(25, 20, '4', 0), (0, 21, '1', 0)], 
    [(26, 9, '4', 1), (1, 10, '1', 1), (10, 11, '2', 2)], 
    [(20, 6, '3', 4), (16, 7, '3', 0)]
]

def test(
    input_file: str = TEST_INPUT, 
    output_file: str = TEST_OUTPUT, 
    queries: list[str] = TEST_QUERIES, 
    list_output_file: str = TEST_LIST_OUTPUT,
    expected_count_data: list[int] = EXPECTED_COUNT_DATA,
    expected_extract_data: list[list] = EXPECTED_EXTRACT_DATA,
    expected_locate_data: list[list] = EXPECTED_LOCATE_DATA,
):
    # creating the suffix array
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
        42,
    )
    suffix_array = _pylibsufr.PySuffixArray(sufr_builder_args)

    # listing; only works if it's the first operation done to the suffix array?
    list_options = _pylibsufr.PyListOptions(
        [],
        True,
        True,
        True,
        None,
        None,
        list_output_file,
    )
    suffix_array.list(list_options)
    with open(list_output_file, 'r') as f:
        print(f"list_results: {f.read()}")

    # counting
    count_options = _pylibsufr.PyCountOptions(
        queries,
        None,
        False,
    )
    count_results = suffix_array.count(count_options)
    count_data = [res.count for res in count_results]
    assert count_data == expected_count_data

    # extracting
    extract_options = _pylibsufr.PyExtractOptions(
        queries,
        None,
        False,
        None,
        None,
    )
    extract_results = suffix_array.extract(extract_options)
    extract_data = [[(seq.suffix, seq.rank, seq.sequence_name, seq.sequence_start, seq.sequence_range, seq.suffix_offset) for seq in res.sequences] for res in extract_results]
    assert extract_data == expected_extract_data

    # locating
    locate_options = _pylibsufr.PyLocateOptions(
        queries,
        None,
        False,
    )
    locate_results = suffix_array.locate(locate_options)
    locate_data = [[(pos.suffix, pos.rank, pos.sequence_name, pos.sequence_position) for pos in res.positions] for res in locate_results]
    assert locate_data == expected_locate_data

    # metadata
    metadata = suffix_array.metadata()
    metadata_attrs = dir(metadata)[-13:]
    metadata_vals = [metadata.filename, metadata.modified, metadata.file_size, metadata.file_version, metadata.is_dna, metadata.allow_ambiguity, metadata.ignore_softmask, metadata.text_len, metadata.len_suffixes, metadata.num_sequences, metadata.sequence_starts, metadata.sequence_names, metadata.sort_type]
    for kvp in map(lambda x: f"{x[0]}:{x[1]}", zip(metadata_attrs, metadata_vals)):
        print(kvp)

    # string_at
    for res in locate_results:
        print(f"{res.query_num} {res.query}")
        for pos in res.positions:
            idx = pos.sequence_position
            seq = suffix_array.string_at(idx, None)
            print(f"\t{idx} {seq}")