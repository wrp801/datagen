# Datagen 
Datagen is a CLI tool to generate random datasets of common SQL data types. This runs in parallel for efficient file creation, and can create either single or multiple CSV or Parquet files. 


## Installation
To install, run the following:

```bash
cargo install --git https://github.com/wrp801/datagen.git
```

## Usage
In the directory where the binary was installed, run the following to see the options

`datagen -h`


##### Create 

To create a csv file named `random` with 10,000 rows, run the following:
```bash
datagen create --rows 10000 --file-type csv --filename random
```

To create 4 parquet files, each with 50,000 rows, prefixed with the name `random`, run the following:

```bash
datagen create --filename random --rows 50000 --file-type parquet -m 4
```

#### Convert 
To convert a csv file to parquet file, run the following:

```bash
datagen convert -s mycsvfile.csv -y parquet
```

To convert a parquet file to csv, run the following: 

```bash
datagen convert -s myparquetfile.parquet -y csv
```





