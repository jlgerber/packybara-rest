# Example usage

## retrieve versionpin

```
curl -X GET http://localhost:8000/v1/versionpin/houdini?level=bayou&role=fx_beta&site=hyderabad&platform=cent7_64
```
## Writing out a packages.xml
```
curl -d '{"show":"facility", "path":"/tmp/packages.xml"}' -H "Content-Type: application/json" -X POST http://localhost:8000/v1/packages.xml
```