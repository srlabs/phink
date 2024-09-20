use crate::fuzzer::parser::Message;

pub struct EnvironmentBuilder();

impl EnvironmentBuilder {
    pub fn new(messages: Vec<Message>) {}
    pub fn toz() {
        // let phink_file =
        // PhinkFiles::new(self.ziggy_config.config.fuzz_output.unwrap_or_default());
        //
        // fs::create_dir_all(phink_file.path(CorpusPath))?;
        // let mut dict_file = fs::File::create(phink_file.path(DictPath))?;
        //
        // write_dict_header(&mut dict_file)?;
        //
        // for (i, selector) in selectors.iter().enumerate() {
        //     write_corpus_file(i, selector, phink_file.path(CorpusPath))?;
        //     write_dict_entry(&mut dict_file, selector).unwrap();
        // }
        //
        // Ok(())
    }
}
