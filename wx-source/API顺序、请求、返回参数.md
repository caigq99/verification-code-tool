1、 https://app.yan.vin/XiaoChengXu/notice.json

响应：[
  {
    "content": "目前验证码获取已经恢复，获取失败的请看文档"
  },
  {
    "content": "获取广告失败的请查看文档，查看文档里的解决办法"
  },
  {
    "content": "偷偷告诉你，彩蛋里好像是个 YAugment 的配置呢"
  },
  {
    "content": "YAugment这个霉啥用的软件已发布，快来试试吧"
  }
]

2、https://app.yan.vin/api/public/signature-key

响应：{
  "key": "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210",
  "timestamp": 1754731929,
  "version": 1
}

3、https://app.yan.vin/api/public/ad-session

请求体：{
  "project_id": "YCursor",
  "device_fingerprint": "31a6e8781988e2c72b5d67e0ac3"
}

响应：{
  "created_at": "2025-08-09T17:32:14.528099",
  "expires_at": "2025-08-09T17:42:14.528092",
  "project_id": "YCursor",
  "session_id": "3b6c42e1-1975-4043-8c05-7a39bdecda45"
}

4、https://app.yan.vin/api/public/ad-verify

请求体：{
  "session_id": "3b6c42e1-1975-4043-8c05-7a39bdecda45",
  "watch_duration": 33497,
  "completion_proof": "miniprogram_ad_completed"
}

响应：{
  "key_expires_at": "2025-08-09T17:37:48.161396",
  "session_id": "3b6c42e1-1975-4043-8c05-7a39bdecda45",
  "temp_token": "0iqho4gp+lbPHnEHYktqOotfJ0pOIkOOvtXhIc3pC8k=",
  "verified": true,
  "verified_at": "2025-08-09T17:32:48.161388"
}

5、https://app.yan.vin/api/public/signature-key

响应：{
  "key": "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210",
  "timestamp": 1754731970,
  "version": 1
}

6、https://app.yan.vin/api/public/verification-code

请求体：{
  "project_id": "YCursor",
  "session_id": "3b6c42e1-1975-4043-8c05-7a39bdecda45",
  "temp_token": "0iqho4gp+lbPHnEHYktqOotfJ0pOIkOOvtXhIc3pC8k="
}

响应：{
  "data": "YAN_TEMP_C0qDSBmFLY1dtG/uljBLhkz8J8sh45jAz+1lKO5IwV1IGlSjBRh4FzlXjtBBB0lHjvtU4y+et9WFoK73CbcFfl9q26lniJyF6vAZyRweaPmw7/xpKLOdVak9BteldZAEGJFvrOpSdWRJQiDs9UyQ1yq5gRZVpgrBpAuhyR53S+Cd8anPnuldZ2MPG7s2244c8jia0w0a1T+g/U1dZkFl3g==_END"
}

最后显示的验证码：9JEMTZ



