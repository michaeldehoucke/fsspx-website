# 1. Build
FROM rust:slim AS builder
# Install native dependencies needed by some crates
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# 2. Runtime
FROM debian:stable-slim
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/fsspx-website /usr/local/bin/
COPY .env .
COPY migrations ./migrations
EXPOSE 8080
CMD ["fsspx-website"]


ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQCybu6nCEjRC6E6E/MdD1TCK4TcjcE3Vt2W+9+5UpTM+rc9gDi9Po9ptd2gNra/MBB6zNO8o59ZxT2m46wnrGwckgnYzKFz0z4Jg/+F17Wk2Tp/DRBYhhcAH8SG8kNbKEHLOgOcEx25MIwKG+GvAsiuC3vxW23h/EtP1TUpu/siGHeRhOAvTZhWuK1HE7uyzlJPoBE+ZGQaIMOpucoLQAVKKhIuWEO2T/kUuaHsvBsSxEJxJI8XZiM6Amif3dA8cy2TIhJqhKppK2dU9spXndZk3dLy/ydbmdvP/Ea5XTB9hkiKtnD2XwYDvNv2kY3aBHef8d1o46XMbDumVeX/xcoWthtQI0ZoUFaIL/Rilry6xAO6NFWyr/eLMsWd7RpHJ3HofHnsKMMbQZ6dO2XqOON6MXoYuBniCFqRkefE93ZLTUszw4tbMcXmPCiBlRebRxNMRE3W7vdU6Fhjbo73R6PIZfpfcAil8G0oI6r5MspoAxbywatQnwBfWoptIU1k8Qq5x4OckA6TSCm5dGJx8YRGNk3Tlpn5oeL3aO5gpojpl4f2lhisyD9MgEfNQXMHHNmN/oyeYJ8g1RhE7p7jVq0nOK5Tl+JG60FKUV/wA0GeexTMI20gYFF+6VtSdoYfzthu0xmg/ZaFP3ZsoY7dJt776tXeVvgrHNm05F8Hj8Y5kQ== michael.dehoucke@gmail.com