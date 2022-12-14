path=$(pwd)/$0
dirpath=$(dirname $path)
echo $dirpath

echo "Downloading openfood.json"
mkdir -p $dirpath/data/openfood
wget https://zenodo.org/record/7305505/files/openfood.json -O $dirpath/data/openfood/openfood.json

echo "Downloading ast.json"
mkdir -p $dirpath/data/ast/
wget https://zenodo.org/record/7229269/files/ast.json -O $dirpath/data/ast/ast.json

echo "Downloading twitter.json"
mkdir -p $dirpath/data/twitter
wget  https://zenodo.org/record/7229287/files/twitter.json -O $dirpath/data/twitter/twitter.json

echo "Downloading crossref.tar.gz"
wget  https://zenodo.org/record/7343312/files/crossref.tar.gz -O $dirpath/data/crossref.tar.gz
echo "Extracting crossref.tar.gz"
cd data;tar xvfz crossref.tar.gz;rm crossref.tar.gz
