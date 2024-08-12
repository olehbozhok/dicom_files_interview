use super::dicom_file_reader::DicomFileData;
use crate::app_config::OutputFormat;
use anyhow::{Context, Ok, Result};
use std::io::Write;

pub trait DisplayFormat {
    fn display_headers(&self, _output: &mut dyn Write) -> Result<()> {
        Ok(())
    }

    fn display_element(&self, data: &DicomFileData, output: &mut dyn Write) -> Result<()>;
}

pub struct TextDisplay;

impl DisplayFormat for TextDisplay {
    fn display_element(&self, data: &DicomFileData, output: &mut dyn Write) -> Result<()> {
        writeln!(
            output,
            "Patient ID: {}\nPatient Name: {}\nFile Path: {}",
            data.patient_id,
            data.patient_name,
            data.path.display()
        )
        .context("could not write result text data of DicomFileData")?;
        Ok(())
    }
}

pub struct CsvDisplay;

impl DisplayFormat for CsvDisplay {
    fn display_headers(&self, output: &mut dyn Write) -> Result<()> {
        let mut writer = csv::Writer::from_writer(output);

        writer
            .write_record(&["Patient ID", "Patient Name", "File Path"])
            .context("could not write csv headers")?;
        Ok(())
    }

    fn display_element(&self, data: &DicomFileData, output: &mut dyn Write) -> Result<()> {
        let mut writer = csv::Writer::from_writer(output);

        let path = data.path.to_string_lossy().to_string();
        writer
            .write_record(&[
                data.patient_id.as_str(),
                data.patient_name.as_str(),
                path.as_str(),
            ])
            .context("could not write result csv data of DicomFileData")?;
        Ok(())
    }
}

pub struct DisplayStrategy(Box<dyn DisplayFormat>);

impl DisplayStrategy {
    pub fn new(format: OutputFormat) -> DisplayStrategy {
        let display_format: Box<dyn DisplayFormat> = match format {
            OutputFormat::Text => Box::new(TextDisplay),
            OutputFormat::Csv => Box::new(CsvDisplay),
        };
        DisplayStrategy(display_format)
    }
}

impl DisplayFormat for DisplayStrategy {
    fn display_headers(&self, output: &mut dyn Write) -> Result<()> {
        self.0.display_headers(output)
    }

    fn display_element(&self, data: &DicomFileData, output: &mut dyn Write) -> Result<()> {
        self.0.display_element(data, output)
    }
}
