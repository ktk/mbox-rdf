#!/bin/bash
set -e

# ktk@netlabs.org (22.8 GiB)
echo "Processing ktk@netlabs.org..."
cargo run --release -- /Users/ktk/Library/Thunderbird/Profiles/4q1gev4s.default-release/ImapMail/mail.netlabs.org/INBOX \
  --data-iri https://data.ktk.netlabs.org/mbox/org/netlabs/ktk/ \
  --graph-iri urn:email:ktk@netlabs.org \
  --output qlever/ktk@netlabs.org.nq.gz

# adrian.gschwend@zazuko.com (9.6 GiB)
echo "Processing adrian.gschwend@zazuko.com..."
cargo run --release -- /Users/ktk/Library/Thunderbird/Profiles/4q1gev4s.default-release/ImapMail/imap.fastmail.com/INBOX \
  --data-iri https://data.ktk.netlabs.org/mbox/com/zazuko/adrian.gschwend/ \
  --graph-iri urn:email:adrian.gschwend@zazuko.com \
  --output qlever/adrian.gschwend@zazuko.com.nq.gz

# adrian@qleverize.com (336.6 MiB)
echo "Processing adrian@qleverize.com..."
cargo run --release -- /Users/ktk/Library/Thunderbird/Profiles/4q1gev4s.default-release/ImapMail/imap.fastmail-2.com/INBOX \
  --data-iri https://data.ktk.netlabs.org/mbox/com/qleverize/adrian/ \
  --graph-iri urn:email:adrian@qleverize.com \
  --output qlever/adrian@qleverize.com.nq.gz

# adrian@qlevia.com (123.4 MiB)
echo "Processing adrian@qlevia.com..."
cargo run --release -- /Users/ktk/Library/Thunderbird/Profiles/4q1gev4s.default-release/ImapMail/imap.fastmail-1.com/INBOX \
  --data-iri https://data.ktk.netlabs.org/mbox/com/qlevia/adrian/ \
  --graph-iri urn:email:adrian@qlevia.com \
  --output qlever/adrian@qlevia.com.nq.gz

echo "All conversions complete. Rebuilding QLever index..."

cd qlever
qlever stop
qlever index --overwrite-existing
qlever start

echo "Done! QLever is running with the updated index."
