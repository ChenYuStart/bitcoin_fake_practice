

impl Codec for GenericCodec {
    type Protocol = GenericProtocol;
    type Request = Vec<u8>;
    type Response = ResponseType;

    async fn read_request<T>(&mut self, _: &Self::Protocol, mut io: &mut T,)
        -> io::Result<Self::Request> where T: AsyncRead + Unpin + Send, {
        let length = unsigned_varint::aio::read_usize(&mut io).await
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

        if length > self.max_request_size {
            return Err(io::Error::new(io::ErrorKind::InvalidInput,
                format!(
                    "Request size exceeds limit: {} > {}",
                    length, self.max_request_size
                ),
            ));
        }

        let mut buffer = vec![0; length];
        io.read_exact(&mut buffer).await?;
        Ok(buffer)
    }

    async fn read_response<T>(&mut self, _: &Self::Protocol, mut io: &mut T,)
        -> io::Result<Self::Response> where T: AsyncRead + Unpin + Send, {
        let length = match unsigned_varint::aio::read_usize(&mut io).await {
            Ok(l) => l,
            Err(unsigned_varint::io::ReadError::Io(err))
                if matches!(err.kind(), io::ErrorKind::UnexpectedEof) =>
            {
                return Ok(Err(()))
            }
            Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidInput, err)),
        };

        if length > self.max_request_size {
            return Err(io::Error::new(io::ErrorKind::InvalidInput,
                format!(
                    "Response size exceeds limit: {} > {}",
                    length, self.max_response_size
                ),
            ));
        }

        let mut buffer = vec![0; length];
        io.read_exact(&mut buffer).await?;
        Ok(Ok(buffer))
    }

    async fn write_request<T>(&mut self, _: &Self::Protocol, io: &mut T, req: Self::Request,)
        -> io::Result<()> where T: AsyncWrite + Unpin + Send, {
        if req.len() > self.max_request_size {
            return Err(io::Error::new(io::ErrorKind::InvalidInput,
                format!(
                    "Request size exceeds limit: {} > {}",
                    req.len(), self.max_request_size
                ),
            ));
        }

        {
            let mut length = unsigned_varint::encode::usize_buffer();
            io.write_all(unsigned_varint::encode::usize(req.len(), &mut length)).await?;
        }

        io.write_all(&req).await?;

        io.close().await?;
        Ok(())
    }

    async fn write_response<T>(&mut self, _: &Self::Protocol, io: &mut T,
        res: Self::Response,) -> io::Result<()> where T: AsyncWrite + Unpin + Send, {
        if let Ok(res) = res {
            if res.len() > self.max_request_size {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Response size exceeds limit: {} > {}",
                        res.len(),
                        self.max_request_size
                    ),
                ));
            }

            {
                let mut length = unsigned_varint::encode::usize_buffer();
                io.write_all(unsigned_varint::encode::usize(res.len(), &mut length)).await?;
            }

            io.write_all(&res).await?;
        }

        io.close().await?;
        Ok(())
    }
}