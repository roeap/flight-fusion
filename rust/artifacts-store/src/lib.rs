use object_store::DynObjectStore;

pub struct MlflowArtifacts {
    object_store: Arc<DynObjectStore>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
