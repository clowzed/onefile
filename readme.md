# onefile self-hosted server
[![License: MIT](https://img.shields.io/badge/License-MIT-gree.svg)](https://opensource.org/licenses/MIT)

![build](https://github.com/clowzed/onefile/actions/workflows/build.yml/badge.svg)


## Techologies
- rust
- actix-web
- docker
- docker-compose

## License
This project is licensed under the MIT License.

## Author
This server was developed by clowzed. If you have any questions or concerns feel free to get in touch with me at clowzed.work@gmail.com.

## Motivation
This project was developed to make it easy for users to upload a single file and access it again later, by providing a key. This is particularly useful for generated HTML reports, where you may want to share the report with someone else, but don't want to give them permanent access to the server. 

## Installation

### Using Docker Compose

1. Clone the repository to your local machine:
```
git clone https://github.com/clowzed/onefile.git
```
2. Navigate to the cloned directory:
```
cd onefile
```
3. Set the `UPLOAD_FOLDER` and `PORT` environment variables in `docker-compose.yml`

4. Start the server using Docker Compose:
```
docker-compose up -d
```

### Manual Installation

1. Clone the repository to your local machine:
```
git clone https://github.com/clowzed/onefile.git
```
2. Navigate to the cloned directory:
```
cd onefile
```
3. Install Rust if it is not already installed. Instructions can be found [here](https://www.rust-lang.org/tools/install).
4. Set the `UPLOAD_FOLDER` and `PORT` environment variables in your shell. 
```
export UPLOAD_FOLDER=/path/to/upload/folder PORT=8080
```
5. Run the project:
```
cargo run --release
```

## Environment Variables

### `UPLOAD_FOLDER`
The `UPLOAD_FOLDER` environment variable specifies the directory where files should be saved by the server. This folder must be writable by the user running the server. 

### `PORT`
The `PORT` environment variable specifies the port on which the server should listen for incoming requests. 

## API Endpoints

### `POST /upload`
This endpoint is used to upload a file. The server will return a key that can be used to access the file later. 
```
curl -F file=@/full/path/to/file http://127.0.0.1:8080/upload 
```

#### Request Parameters
- `file` - The file to be uploaded. 

#### Response
- `key` - The key that can be used to access the uploaded file. 

### `GET /get/{key}`
This endpoint is used to retrieve a file using the key provided when the file was uploaded. 

#### Request Parameters
- `key` - The key used to access the desired file. 

#### Response
- The contents of the requested file.
