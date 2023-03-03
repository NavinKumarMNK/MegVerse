use ndarray::{Array1, Array2};
use ndarray_npy::ReadNpyExt;
use pickle::{PickleFile, PickleValue};
use rust_lightgbm::{Booster, Dataset, Predictor};

fn run_lightgbm_inference(input_data: Array2<f32>) -> Vec<f32> {
    // Load pre-trained LightGBM model from pickle file
    let model_file = include_bytes!("path/to/model.pkl");
    let mut model_pickle = PickleFile::new(model_file);
    let model = if let PickleValue::Tuple(model) = model_pickle.load().unwrap() {
        // Extract model and parameter dictionaries from the pickle file
        let model_dict = model[1].as_dict().unwrap();
        let param_dict = model[2].as_dict().unwrap();

        // Extract relevant parameters for creating Booster and Dataset
        let num_trees = param_dict.get("num_trees").unwrap().as_i32().unwrap() as u32;
        let num_classes = param_dict.get("num_classes").unwrap().as_i32().unwrap() as u32;
        let feature_names = param_dict.get("feature_names").unwrap().as_list().unwrap();
        let feature_names = feature_names.iter().map(|x| x.as_string().unwrap()).collect::<Vec<_>>();

        // Create Booster and load model parameters
        let mut booster = Booster::new(&[]);
        booster.set_params(model_dict);
        for _ in 0..num_trees {
            booster.add_tree(model_dict);
        }

        // Create Dataset for input data
        let mut dataset = Dataset::from_array2(&input_data);
        dataset.set_feature_names(&feature_names);

        // Create Predictor for performing inference
        Predictor::new(num_classes, &booster, dataset).unwrap()
    } else {
        panic!("Invalid LightGBM model file");
    };

    // Perform inference using input data
    model.predict(&input_data).unwrap()
}
