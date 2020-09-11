use analyzeme::testing_common::run_end_to_end_serialization_test;
use measureme::FileSerializationSink;

#[test]
fn test_file_serialization_sink_1_thread() {
    run_end_to_end_serialization_test::<FileSerializationSink>(
        "file_serialization_sink_test_1_thread",
        1,
    );
}

#[test]
fn test_file_serialization_sink_8_threads() {
    run_end_to_end_serialization_test::<FileSerializationSink>(
        "file_serialization_sink_test_8_threads",
        8,
    );
}
