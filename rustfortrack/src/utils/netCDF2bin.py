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
    matrix = np.nan_to_num(matrix, nan=-9999)
    # Save the matrix in binary format
    matrix.tofile(file.replace('.nc', '.bin'))

    # Mount output path
    output_file = output_path + pathlib.Path(file).name.replace('.nc', '.bin')

    # Save the matrix in binary format compressed
    matrix.tofile(output_file, format='f4')
