sudo apt-get update
sudo apt install -y postgresql git gcc make tmux pkg-config openssl libssl-dev libpq-dev apache2

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
git clone https://github.com/leohscl/wikiguesser
source ~/.bashrc
cd wikiguesser
sudo cp conf_amazon/apache_default.conf /etc/apache2/sites-available/000-default.conf
ip=$(dig +short myip.opendns.com @resolver1.opendns.com)
sed -i "s/13.39.48.179/$ip/g" .env_amazon
cp .env_amazon .env
cd wiki_random/
export DATABASE_URL="postgres://postgres:wikiguesser@database-2.c2zgka3ai9ya.eu-west-3.rds.amazonaws.com:5432/wikiguesser"

cargo install diesel_cli --no-default-features --features postgres
bash diesel_setup.sh
cargo run --release
#rm -r target




cd ../frontend/

cargo install trunk

rustup target add wasm32-unknown-unknown
trunk build --release
sudo cp dist/* /var/www/html/
#rm -r target

sudo a2enmod proxy_http
sudo a2enmod rewrite

sudo systemctl restart apache2



cd ../backend/
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.deb.sh | sudo bash
sudo apt-get install git-lfs
git lfs install
git lfs pull
cargo run --release
