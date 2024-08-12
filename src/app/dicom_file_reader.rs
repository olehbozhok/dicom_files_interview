use dicom::core::value::ConvertValueError;
use std::path::PathBuf;
use thiserror::Error;

use dicom::dictionary_std::tags;
use dicom::object::{open_file, AccessError, DicomObject, FileDicomObject, ReadError};

#[derive(Error, Debug)]
pub enum DicomError {
    #[error("could not read dicom file: {0}")]
    Read(#[from] ReadError),

    #[error("could not handle patient name: {0}, path: {1}")]
    PatientName(PatientNameErr, PathBuf),

    #[error("could not handle patient id: {0}, path: {1}")]
    PatientID(PatientIDErr, PathBuf),
}

#[derive(Error, Debug)]
pub enum PatientNameErr {
    #[error("could not read dicom element: {0}")]
    Access(#[from] AccessError),
    #[error("could not read dicom element: {0}")]
    ConvToStr(#[from] ConvertValueError),
}

#[derive(Error, Debug)]
pub enum PatientIDErr {
    #[error("could not read dicom element: {0}")]
    Access(#[from] AccessError),
    #[error("could not read dicom element: {0}")]
    ConvToStr(#[from] ConvertValueError),
}

#[derive(Debug)]
pub struct DicomFileData {
    pub patient_id: String,
    pub patient_name: String,
    pub path: PathBuf,
}

type File = FileDicomObject<dicom::object::InMemDicomObject>;

fn get_patient_name(obj: &File) -> Result<String, PatientNameErr> {
    let patient_name = obj.element(tags::PATIENT_NAME)?.to_str()?;
    Ok(patient_name.to_string())
}

fn get_patient_id(obj: &File) -> Result<String, PatientIDErr> {
    let patient_name = obj.element(tags::PATIENT_ID)?.to_str()?;
    Ok(patient_name.to_string())
}

pub fn handle_file(path: PathBuf) -> Result<DicomFileData, DicomError> {
    let obj = open_file(&path)?;
    Ok(DicomFileData {
        patient_id: get_patient_id(&obj).map_err(|err| DicomError::PatientID(err, path.clone()))?,
        patient_name: get_patient_name(&obj)
            .map_err(|err| DicomError::PatientName(err, path.clone()))?,
        path,
    })
}
