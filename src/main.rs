use zarr::prelude::*;
use zarr::smallvec::smallvec;

fn zarr_roundtrip(root_path: &str) -> std::io::Result<()> {
    let n = FilesystemHierarchy::open_or_create(root_path)?;
    let q = 1;
    let _w = "qwdqdq";
        
    
    let chunk_shape = smallvec![44, 33, 22];                                         //
    let array_meta = ArrayMetadata::new(                                             //
        smallvec![100, 200, 300],                                                    //
        chunk_shape,                                                                 //
        i16::ZARR_TYPE,                                                              //
        CompressionType::default(),                                                  //
    );                                                                               //
    let chunk_data = vec![0i16; array_meta.get_chunk_num_elements()];                //
   
    let chunk_in = SliceDataChunk::new(                                              //
        smallvec![0, 0, 0],                                                          //
        &chunk_data);                                                                //
                                                                                     //
    let path_name = "";                                                              //
                                                                                     //
    n.create_array(path_name, &array_meta)?;                                         //
    n.write_chunk(path_name, &array_meta, &chunk_in)?;                               //
    let chunk_out = n.read_chunk::<i16>(path_name, &array_meta, smallvec![0, 0, 0])? //
        .expect("Chunk is empty");                                                   //
    assert_eq!(chunk_out.get_data(), &chunk_data[..]);                               //
    

    Ok(())
}

fn main() {
    zarr_roundtrip("t/home/mat/gh/vecview/0005.zarr").expect("Zarr roundtrip failed!");
   // std::fs::remove_dir_all("tmp.zr3").expect("Failed to delete temporary zarr hierarchy");
}
