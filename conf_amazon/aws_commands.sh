EC2
ssh -i "/home/lhenches/.ssh/aws/t430.pem" ec2-user@ec2-35-180-111-60.eu-west-3.compute.amazonaws.com
scp -i "/home/lhenches/.ssh/aws/t430.pem" ~/.ssh/id_rsa ec2-user@ec2-35-180-111-60.eu-west-3.compute.amazonaws.com:~/.ssh/


RDS
rds instance
username=postgres
password=wikiguesser

sudo amazon-linux-extras install -y postgresql10
sudo yum install -y postgresql-devel
sudo yum install -y git
sudo yum install -y docker
sudo yum install -y gcc gcc-c++ make
sudo yum install -y openssl-devel
git clone https://github.com/leohscl/wikiguesser
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
cd wikiguesser/wiki_random
cargo run --release

cd wikiguesser/backend
cargo install diesel_cli ?
cargo install diesel_cli --no-default-features --features postgres ?

# ubuntu
 
sudo apt-get update
sudo apt install -y postgresql git gcc make tmux pkg-config openssl libssl-dev libpq-dev apache2
git clone https://github.com/leohscl/wikiguesser

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
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



sudo a2enmod ssl
sudo a2enmod proxy_http
sudo a2enmod rewrite
sudo sh -c 'echo "
<Directory /var/www/html>
  AllowOverride ALL 
</Directory>" >> /etc/apache2/apache2.conf'
# run certbot 
# remove prompts ?
sudo certbot certonly --apache
# sudo mkdir /etc/apache2/certs
# cd /etc/apache2/certs
#sudo openssl req -new -newkey rsa:4096 -x509 -sha256 -days 365 -nodes -out apache.crt -keyout apache.keyout \
#-subj "/C=FR/ST=PARIS/L=PARIS /O=wikitrouve/CN=wikitrouve.fr/emailAddress=leo.henches@gmail.com"

sudo systemctl restart apache2



cd ../backend/
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.deb.sh | sudo bash
sudo apt-get install git-lfs
git lfs install
git lfs pull
cargo run --release
