path=$(pwd)/$0
dirpath=$(dirname $path)

echo $dirpath

echo "Downloading ast.json"
mkdir -p $dirpath/data/ast/
wget https://zenodo.org/record/7225575/files/ast.json -O $dirpath/data/ast/ast.json

echo "Downloading twitter.json"
mkdir -p $dirpath/data/twitter
wget  https://zenodo.org/record/7225577/files/twitter.json -O $dirpath/data/twitter/twitter.json
echo "Downloading crossref.tar.gz"
mkdir -p $dirpath/data/crossref
wget  https://zenodo.org/record/7225594/files/crossref.tar.gz -O $dirpath/data/crossref/crossref.tar.gz
echo "Extracting crossref.tar.gz"
cd $dirpath/data/crossref;tar xvfz crossref.tar.gz;rm crossref.tar.gz
