FROM node:20.19.2-alpine3.22 AS builder

WORKDIR /code

COPY package.json package-lock.json /code/
RUN npm ci                          # ← just this, no audit fix
COPY . /code/
RUN npm run build && rm -rf .next/cache

FROM node:20.19.2-alpine3.22 AS worker

WORKDIR /code
ENV NODE_ENV=production
COPY --from=builder /code/package.json ./
COPY --from=builder /code/package-lock.json ./
COPY --from=builder /code/.next ./.next
COPY --from=builder /code/public ./public

RUN npm ci --omit=dev && npm cache clean --force

CMD ["npm", "start"]