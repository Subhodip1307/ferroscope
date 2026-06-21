FROM node:26-alpine AS builder

WORKDIR /code

COPY package.json package-lock.json /code/
RUN npm ci
COPY . /code/
RUN npm run build && rm -rf .next/cache

FROM node:26-alpine AS worker

WORKDIR /code
ENV NODE_ENV=production

COPY --from=builder /code/.next/standalone ./
COPY --from=builder /code/.next/static ./.next/static
COPY --from=builder /code/public ./public

CMD ["node", "server.js"]