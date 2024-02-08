FROM python:3.11-slim
WORKDIR /src/
COPY ./build/auto_execute/requirements.txt /src/requirements.txt
RUN pip install -r requirements.txt
COPY ./src/ /src/
CMD ["python","auto_execute.py"]