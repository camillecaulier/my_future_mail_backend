## How to test the code 
``cargo test```
## How to add packages 
`cargo add <package name>`

## run locally the email endpoint

curl -X POST http://127.0.0.1:3000/send_email \
     -H "Content-Type: application/json" \
     -d '{"to": "camillecaulier.cc@gmail.com", "message": "Test email message"}'