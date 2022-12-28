import netCDF4
import glob
import numpy as np
import pathlib

input_path = '/home/helvecioneto/pyfortrack/data/simple_example/**/'
output_path = '/home/helvecioneto/rustfortrack/rustfortrack/input/'

for file in glob.glob(input_path + '*.nc'):
    # Open the file
    nc = netCDF4.Dataset(file, 'r')
    # Open matrix from variable DBZc
    matrix = nc.variables['DBZc'][:].data
    # Transform nan to -9999
    # matrix = np.nan_to_num(matrix, nan=-9999)

    # Get only the first 2 dimensions
    matrix = matrix[0, 0, :, :]

    # Mount output path
    output_file = output_path + pathlib.Path(file).name.replace('.nc', '.bin')

    # Convert to float32
    matrix = matrix.astype(np.float32)

    # Replace nan to -9999
    matrix[np.isnan(matrix)] = -9999

    print(np.unique(matrix))

    # Save the matrix in binary format f64
    matrix.tofile(output_file, format='%f')
