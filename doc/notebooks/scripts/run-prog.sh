ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.89.70.163 "touch tmsn/neighbors.txt; cd tmsn; git checkout dev; git pull; ps aux | grep tmsn | awk '{print \$2}' | xargs kill" &

ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@100.24.1.124 "touch tmsn/neighbors.txt; cd tmsn; git checkout dev; git pull; ps aux | grep tmsn | awk '{print \$2}' | xargs kill" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@54.224.251.171 "touch tmsn/neighbors.txt; cd tmsn; git checkout dev; git pull; ps aux | grep tmsn | awk '{print \$2}' | xargs kill" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@54.162.20.161 "touch tmsn/neighbors.txt; cd tmsn; git checkout dev; git pull; ps aux | grep tmsn | awk '{print \$2}' | xargs kill" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.88.50.119 "touch tmsn/neighbors.txt; cd tmsn; git checkout dev; git pull; ps aux | grep tmsn | awk '{print \$2}' | xargs kill" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@34.238.127.28 "touch tmsn/neighbors.txt; cd tmsn; git checkout dev; git pull; ps aux | grep tmsn | awk '{print \$2}' | xargs kill" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.81.150.141 "touch tmsn/neighbors.txt; cd tmsn; git checkout dev; git pull; ps aux | grep tmsn | awk '{print \$2}' | xargs kill" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@35.153.204.10 "touch tmsn/neighbors.txt; cd tmsn; git checkout dev; git pull; ps aux | grep tmsn | awk '{print \$2}' | xargs kill" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.93.69.63 "touch tmsn/neighbors.txt; cd tmsn; git checkout dev; git pull; ps aux | grep tmsn | awk '{print \$2}' | xargs kill" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.93.54.145 "touch tmsn/neighbors.txt; cd tmsn; git checkout dev; git pull; ps aux | grep tmsn | awk '{print \$2}' | xargs kill" &

wait

# Start test
echo "now start building"

ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.89.70.163 "cd tmsn; cargo build --release " &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@100.24.1.124 "cd tmsn; cargo build --release " &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@54.224.251.171 "cd tmsn; cargo build --release " &

ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@54.162.20.161 "cd tmsn; cargo build --release " &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.88.50.119 "cd tmsn; cargo build --release " &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@34.238.127.28 "cd tmsn; cargo build --release " &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.81.150.141 "cd tmsn; cargo build --release " &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@35.153.204.10 "cd tmsn; cargo build --release " &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.93.69.63 "cd tmsn; cargo build --release " &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.93.54.145 "cd tmsn; cargo build --release " &

wait


echo "now start testing"

ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.89.70.163 "cd tmsn; cargo test --release -- --nocapture stress_test_network_10 > log.txt" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@100.24.1.124 "cd tmsn; cargo test --release -- --nocapture stress_test_network_10 > log.txt" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@54.224.251.171 "cd tmsn; cargo test --release -- --nocapture stress_test_network_10 > log.txt" &

ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@54.162.20.161 "cd tmsn; cargo test --release -- --nocapture stress_test_network_10 > log.txt" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.88.50.119 "cd tmsn; cargo test --release -- --nocapture stress_test_network_10 > log.txt" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@34.238.127.28 "cd tmsn; cargo test --release -- --nocapture stress_test_network_10 > log.txt" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.81.150.141 "cd tmsn; cargo test --release -- --nocapture stress_test_network_10 > log.txt" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@35.153.204.10 "cd tmsn; cargo test --release -- --nocapture stress_test_network_10 > log.txt" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.93.69.63 "cd tmsn; cargo test --release -- --nocapture stress_test_network_10 > log.txt" &
ssh -o "StrictHostKeyChecking=no" -i ~/Dropbox/documents/vault/aws/aws-brain.pem ubuntu@3.93.54.145 "cd tmsn; cargo test --release -- --nocapture stress_test_network_10 > log.txt" &

wait
