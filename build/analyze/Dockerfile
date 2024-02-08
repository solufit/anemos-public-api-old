FROM python:3.11-slim
WORKDIR /src/
COPY ./build/analyze/requirements.txt requirements.txt
RUN pip install -r requirements.txt
COPY ./src /src/
CMD ["uvicorn","analyze:app","--host=0.0.0.0","--port=80"]