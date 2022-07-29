from pydantic import BaseModel


class Global(BaseModel):
    postgresqlSecretName: str
    serviceAccountName: str
