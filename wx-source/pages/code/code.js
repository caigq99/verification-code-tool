require("../../@babel/runtime/helpers/Arrayincludes");

var t = require("../../@babel/runtime/helpers/regeneratorRuntime"), e = require("../../@babel/runtime/helpers/asyncToGenerator"), o = require("../../AE61E3604B3CC87FC8078B67643A8012.js");

Page({
    countdownTimer: null,
    data: {
        verificationCode: "",
        codeCharacters: [],
        codeLoading: !0,
        codeError: !1,
        software: "YCursor",
        softwareInfo: {
            name: "YCursor",
            description: "Y 系列"
        },
        countdown: {
            show: !1,
            timeText: "",
            totalSeconds: 0,
            expired: !1
        },
        hitokoto: {
            text: "加载中...",
            from: "",
            from_who: "",
            loading: !0,
            error: !1
        },
        animationState: {
            isEntering: !0,
            isExiting: !1,
            contentReady: !1,
            animationQueue: []
        },
        systemInfo: {
            statusBarHeight: 44,
            menuButtonHeight: 32,
            menuButtonTop: 0,
            backButtonTop: 0,
            backButtonLeft: 16
        },
        sessionId: "",
        sessionExpiresAt: "",
        tempToken: "",
        showAdPrompt: !1,
        adWatchStartTime: 0,
        apiSignatureSecret: "",
        apiKeyVersion: 0,
        apiKeyLastUpdate: 0
    },
    onLoad: function(t) {
        var e = this;
        this.clearEasterCount(), this.getSystemInfo(), this.initPageAnimation();
        try {
            this.generateDeviceFingerprint();
        } catch (t) {}
        var o = t.software || "YCursor", n = t.sessionId, r = t.tempToken ? decodeURIComponent(t.tempToken) : null;
        this.setSoftwareInfo(o), this.fetchCurrentSignatureKey().then(function() {
            n && r ? (e.setData({
                sessionId: n,
                tempToken: r
            }), e.fetchVerificationCodeWithSession(n, r)) : e.fetchVerificationCode();
        }).catch(function(t) {
            console.error("获取签名密钥失败:", t), n && r ? (e.setData({
                sessionId: n,
                tempToken: r
            }), e.fetchVerificationCodeWithSession(n, r)) : e.fetchVerificationCode();
        }), this.fetchHitokoto();
    },
    setSoftwareInfo: function(t) {
        var e = {
            YCursor: {
                name: "YCursor",
                description: "Y 系列"
            },
            YAugment: {
                name: "YAugment",
                description: "Y 系列"
            }
        }, o = e[t] || e.YCursor;
        this.setData({
            software: t,
            softwareInfo: o
        });
    },
    processVerificationCode: function(t) {
        for (var e = [], o = 0; o < t.length; o++) {
            var n = t[o], r = /[0-9]/.test(n) ? "number" : "letter";
            e.push({
                char: n,
                type: r
            });
        }
        return e;
    },
    clearEasterCount: function() {
        try {
            wx.setStorageSync("easterCount", 0);
        } catch (t) {}
    },
    getSystemInfo: function() {
        var t = wx.getSystemInfoSync(), e = wx.getMenuButtonBoundingClientRect(), o = e.top + 6, n = t.windowWidth - e.right + 8;
        this.setData({
            "systemInfo.statusBarHeight": t.statusBarHeight,
            "systemInfo.menuButtonHeight": e.height,
            "systemInfo.menuButtonTop": e.top,
            "systemInfo.backButtonTop": o,
            "systemInfo.backButtonLeft": n
        });
    },
    generateDeviceFingerprint: function() {
        try {
            var t, e, n = wx.getSystemInfoSync(), r = wx.getAccountInfoSync(), a = {
                model: n.model || "unknown",
                platform: n.platform || "unknown",
                version: n.version || "unknown",
                system: n.system || "unknown",
                language: n.language || "unknown",
                screenWidth: n.screenWidth || 0,
                screenHeight: n.screenHeight || 0,
                windowWidth: n.windowWidth || 0,
                windowHeight: n.windowHeight || 0,
                pixelRatio: n.pixelRatio || 1,
                appId: (null == r || null === (t = r.miniProgram) || void 0 === t ? void 0 : t.appId) || "unknown",
                envVersion: (null == r || null === (e = r.miniProgram) || void 0 === e ? void 0 : e.envVersion) || "unknown",
                timestamp: Date.now(),
                random: Math.random(),
                statusBarHeight: n.statusBarHeight || 0,
                safeArea: n.safeArea ? {
                    top: n.safeArea.top,
                    bottom: n.safeArea.bottom,
                    left: n.safeArea.left,
                    right: n.safeArea.right
                } : null,
                networkType: "unknown",
                benchMark: n.benchmarkLevel || 0,
                brand: n.brand || "unknown"
            };
            try {
                wx.getNetworkType({
                    success: function(t) {
                        a.networkType = t.networkType || "unknown";
                    },
                    fail: function() {
                        a.networkType = "unknown";
                    }
                });
            } catch (t) {
                a.networkType = "unknown";
            }
            var i = JSON.stringify(a);
            return o.SHA256(i).toString();
        } catch (t) {
            try {
                var s = {
                    timestamp: Date.now(),
                    random: Math.random(),
                    userAgent: "miniprogram",
                    fallback: !0
                };
                try {
                    var c = wx.getSystemInfoSync();
                    s.platform = c.platform || "unknown", s.version = c.version || "unknown";
                } catch (t) {
                    s.platform = "unknown", s.version = "unknown";
                }
                var d = JSON.stringify(s);
                return o.SHA256(d).toString();
            } catch (t) {
                var u = Date.now().toString() + Math.random().toString();
                return o.SHA256(u).toString();
            }
        }
    },
    initPageAnimation: function() {
        var t = this;
        wx.nextTick(function() {
            t.setData({
                "animationState.isEntering": !0
            }), setTimeout(function() {
                t.setData({
                    "animationState.contentReady": !0,
                    "animationState.isEntering": !1
                });
            }, 80);
        });
    },
    createAdSession: function(o, n) {
        var r = this;
        return e(t().mark(function e() {
            return t().wrap(function(t) {
                for (;;) switch (t.prev = t.next) {
                  case 0:
                    return t.abrupt("return", new Promise(function(t, e) {
                        var a = {
                            project_id: o,
                            device_fingerprint: n
                        }, i = r.generateSignedHeaders("POST", "/api/public/ad-session", a);
                        wx.request({
                            url: "https://app.yan.vin/api/public/ad-session",
                            method: "POST",
                            header: i,
                            data: a,
                            success: function(o) {
                                200 === o.statusCode && o.data ? o.data.error ? e(new Error(o.data.error)) : t(o.data) : 429 === o.statusCode ? (wx.showModal({
                                    title: "请求频率过高",
                                    content: "频率太高，无法创建会话，请等待一会稍后再试。请不要短时间内频繁重复，避免进风控小黑屋。",
                                    showCancel: !1,
                                    confirmText: "我知道了",
                                    confirmColor: "#ff6b6b"
                                }), e(new Error("Rate limit exceeded"))) : e(new Error("Invalid response"));
                            },
                            fail: function(t) {
                                e(t);
                            }
                        });
                    }));

                  case 1:
                  case "end":
                    return t.stop();
                }
            }, e);
        }))();
    },
    verifyAdCompletion: function(o, n, r) {
        var a = this;
        return e(t().mark(function e() {
            return t().wrap(function(t) {
                for (;;) switch (t.prev = t.next) {
                  case 0:
                    return t.abrupt("return", new Promise(function(t, e) {
                        var i = {
                            session_id: o,
                            watch_duration: n,
                            completion_proof: r || ""
                        }, s = a.generateSignedHeaders("POST", "/api/public/ad-verify", i);
                        wx.request({
                            url: "https://app.yan.vin/api/public/ad-verify",
                            method: "POST",
                            header: s,
                            data: i,
                            success: function(o) {
                                200 === o.statusCode && o.data ? o.data.error ? e(new Error(o.data.error)) : t(o.data) : 429 === o.statusCode ? (wx.showModal({
                                    title: "请求频率过高",
                                    content: "频率太高，无法验证广告观看，请等待一会稍后再试。请不要短时间内频繁重复，避免进风控小黑屋。",
                                    showCancel: !1,
                                    confirmText: "我知道了",
                                    confirmColor: "#ff6b6b"
                                }), e(new Error("Rate limit exceeded"))) : e(new Error("Invalid response"));
                            },
                            fail: function(t) {
                                e(t);
                            }
                        });
                    }));

                  case 1:
                  case "end":
                    return t.stop();
                }
            }, e);
        }))();
    },
    getVerificationCodeWithSession: function(o, n, r) {
        var a = this;
        return e(t().mark(function e() {
            return t().wrap(function(t) {
                for (;;) switch (t.prev = t.next) {
                  case 0:
                    return t.abrupt("return", new Promise(function(t, e) {
                        var i = {
                            project_id: o,
                            session_id: n,
                            temp_token: r
                        }, s = a.generateSignedHeaders("POST", "/api/public/verification-code", i);
                        wx.request({
                            url: "https://app.yan.vin/api/public/verification-code",
                            method: "POST",
                            header: s,
                            data: i,
                            success: function(o) {
                                if (200 === o.statusCode && o.data) o.data.error ? e(new Error(o.data.error)) : t(o.data); else if (400 === o.statusCode) {
                                    var n, r = (null === (n = o.data) || void 0 === n ? void 0 : n.error) || "Session not verified";
                                    e(new Error(r));
                                } else if (401 === o.statusCode) {
                                    var a, i = (null === (a = o.data) || void 0 === a ? void 0 : a.error) || "Invalid signature";
                                    e(new Error(i));
                                } else if (404 === o.statusCode) {
                                    var s, c = (null === (s = o.data) || void 0 === s ? void 0 : s.error) || "Session not found or expired";
                                    e(new Error(c));
                                } else if (429 === o.statusCode) {
                                    var d, u = (null === (d = o.data) || void 0 === d ? void 0 : d.error) || "Rate limit exceeded";
                                    e(new Error(u));
                                } else {
                                    var f, h = (null === (f = o.data) || void 0 === f ? void 0 : f.error) || "Invalid response (".concat(o.statusCode, ")");
                                    e(new Error(h));
                                }
                            },
                            fail: function(t) {
                                e(t);
                            }
                        });
                    }));

                  case 1:
                  case "end":
                    return t.stop();
                }
            }, e);
        }))();
    },
    startThreeStageVerification: function() {
        var o = this;
        return e(t().mark(function e() {
            var n, r;
            return t().wrap(function(t) {
                for (;;) switch (t.prev = t.next) {
                  case 0:
                    return t.prev = 0, n = o.generateDeviceFingerprint(), t.next = 4, o.createAdSession(o.data.software, n);

                  case 4:
                    if ((r = t.sent).session_id) {
                        t.next = 7;
                        break;
                    }
                    throw new Error("Failed to create session: no session_id");

                  case 7:
                    return o.setData({
                        sessionId: r.session_id,
                        sessionExpiresAt: r.expires_at
                    }), o.showRewardedAd(), t.abrupt("return", !0);

                  case 12:
                    throw t.prev = 12, t.t0 = t.catch(0), t.t0;

                  case 15:
                  case "end":
                    return t.stop();
                }
            }, e, null, [ [ 0, 12 ] ]);
        }))();
    },
    handleAdWatchCompleted: function(o) {
        var n = this;
        return e(t().mark(function e() {
            var r, a, i, s, c;
            return t().wrap(function(t) {
                for (;;) switch (t.prev = t.next) {
                  case 0:
                    if (t.prev = 0, r = n.data.sessionId) {
                        t.next = 4;
                        break;
                    }
                    throw new Error("No session ID available");

                  case 4:
                    return t.next = 6, n.verifyAdCompletion(r, o);

                  case 6:
                    if ((a = t.sent).verified && a.temp_token) {
                        t.next = 9;
                        break;
                    }
                    throw new Error("Ad verification failed");

                  case 9:
                    return n.setData({
                        tempToken: a.temp_token
                    }), t.next = 12, n.getVerificationCodeWithSession(n.data.software, r, a.temp_token);

                  case 12:
                    if ((i = t.sent).data) {
                        t.next = 15;
                        break;
                    }
                    throw new Error("Failed to get verification code");

                  case 15:
                    s = n.decryptVerificationData(i.data), c = n.processVerificationCode(s.code), n.setData({
                        verificationCode: s.code,
                        codeCharacters: c,
                        codeLoading: !1,
                        codeError: !1
                    }), s.expires_at && n.startCountdown(s.expires_at), t.next = 24;
                    break;

                  case 21:
                    t.prev = 21, t.t0 = t.catch(0), n.handleCodeError("验证码获取失败，请重试");

                  case 24:
                  case "end":
                    return t.stop();
                }
            }, e, null, [ [ 0, 21 ] ]);
        }))();
    },
    fetchCurrentSignatureKey: function() {
        var t = this;
        return new Promise(function(e, o) {
            var n = Date.now();
            t.data.apiSignatureSecret && n - t.data.apiKeyLastUpdate < 3e5 ? e() : wx.request({
                url: "https://app.yan.vin/api/public/signature-key",
                method: "GET",
                header: {
                    "Content-Type": "application/json",
                    Referer: "https://servicewechat.com/wx421aabd7feefa0ed/devtools/page-frame.html"
                },
                success: function(o) {
                    200 === o.statusCode && o.data ? o.data.key && o.data.version ? (t.setData({
                        apiSignatureSecret: o.data.key,
                        apiKeyVersion: o.data.version,
                        apiKeyLastUpdate: n
                    }), e()) : (console.error("获取签名密钥失败: 响应数据格式错误"), t.setData({
                        apiSignatureSecret: "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210",
                        apiKeyVersion: 1,
                        apiKeyLastUpdate: n
                    }), e()) : (console.error("获取签名密钥失败: HTTP状态码", o.statusCode), t.setData({
                        apiSignatureSecret: "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210",
                        apiKeyVersion: 1,
                        apiKeyLastUpdate: n
                    }), e());
                },
                fail: function(o) {
                    console.error("获取签名密钥网络请求失败:", o), t.setData({
                        apiSignatureSecret: "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210",
                        apiKeyVersion: 1,
                        apiKeyLastUpdate: n
                    }), e();
                }
            });
        });
    },
    generateApiSignature: function(t, e, n, r, a) {
        try {
            var i = "";
            if (n) {
                var s = Object.keys(n).sort(), c = {};
                s.forEach(function(t) {
                    c[t] = n[t];
                }), i = JSON.stringify(c).replace(/\s/g, "");
            }
            var d = "".concat(t.toUpperCase(), "\n").concat(e, "\n").concat(i, "\n").concat(r, "\n").concat(a);
            return o.HmacSHA256(d, this.data.apiSignatureSecret).toString();
        } catch (t) {
            return null;
        }
    },
    generateSignedHeaders: function(t, e, o) {
        this.data.apiSignatureSecret || (console.warn("API签名密钥未初始化，使用默认v1密钥"), this.setData({
            apiSignatureSecret: "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210",
            apiKeyVersion: 1,
            apiKeyLastUpdate: Date.now()
        }));
        var n = Math.floor(Date.now() / 1e3).toString(), r = Math.random().toString(36).substr(2, 16);
        return {
            "Content-Type": "application/json",
            "X-YAN-Signature": this.generateApiSignature(t, e, o, n, r),
            "X-YAN-Timestamp": n,
            "X-YAN-Nonce": r
        };
    },
    fetchVerificationCodeWithSession: function(t, e) {
        var o = this;
        this.setData({
            codeLoading: !0,
            codeError: !1,
            verificationCode: "",
            codeCharacters: []
        });
        var n = {
            project_id: this.data.software,
            session_id: t,
            temp_token: e
        }, r = this.generateSignedHeaders("POST", "/api/public/verification-code", n);
        wx.request({
            url: "https://app.yan.vin/api/public/verification-code",
            method: "POST",
            header: r,
            data: n,
            success: function(t) {
                if (200 === t.statusCode && t.data) if (t.data.error) o.handleCodeError(t.data.error); else if (t.data.data) try {
                    var n = o.decryptVerificationDataWithTempKey(t.data.data, e), r = o.processVerificationCode(n.code);
                    o.setData({
                        verificationCode: n.code,
                        codeCharacters: r,
                        codeLoading: !1,
                        codeError: !1
                    }), n.expires_at && o.startCountdown(n.expires_at);
                } catch (t) {
                    o.handleCodeError("解密失败");
                } else o.handleCodeError("无效的响应数据"); else if (400 === t.statusCode) {
                    var a, i = (null === (a = t.data) || void 0 === a ? void 0 : a.error) || "Session not verified";
                    o.handleCodeError(i);
                } else if (401 === t.statusCode) {
                    var s, c = (null === (s = t.data) || void 0 === s ? void 0 : s.error) || "Invalid signature";
                    o.handleCodeError(c);
                } else if (404 === t.statusCode) {
                    var d, u = (null === (d = t.data) || void 0 === d ? void 0 : d.error) || "Session not found or expired";
                    o.handleCodeError(u);
                } else if (429 === t.statusCode) wx.showModal({
                    title: "请求频率过高",
                    content: "频率太高，无法获取验证码，请等待一会稍后再试。请不要短时间内频繁重复，避免进风控小黑屋。",
                    showCancel: !1,
                    confirmText: "我知道了",
                    confirmColor: "#ff6b6b"
                }), o.handleCodeError("频率限制"); else {
                    var f, h = (null === (f = t.data) || void 0 === f ? void 0 : f.error) || "网络请求失败 (".concat(t.statusCode, ")");
                    o.handleCodeError(h);
                }
            },
            fail: function() {
                o.handleCodeError("网络连接失败");
            }
        });
    },
    fetchVerificationCode: function() {
        var o = this;
        return e(t().mark(function e() {
            return t().wrap(function(t) {
                for (;;) switch (t.prev = t.next) {
                  case 0:
                    return o.setData({
                        codeLoading: !0,
                        codeError: !1,
                        verificationCode: "",
                        codeCharacters: []
                    }), t.prev = 1, t.next = 4, o.startThreeStageVerification();

                  case 4:
                    if (!t.sent) {
                        t.next = 8;
                        break;
                    }
                    return o.setData({
                        codeLoading: !1,
                        showAdPrompt: !0
                    }), t.abrupt("return");

                  case 8:
                    t.next = 13;
                    break;

                  case 10:
                    t.prev = 10, t.t0 = t.catch(1), o.handleCodeError("验证流程启动失败，请重试");

                  case 13:
                  case "end":
                    return t.stop();
                }
            }, e, null, [ [ 1, 10 ] ]);
        }))();
    },
    showRewardedAd: function() {
        var t = this;
        try {
            if (this.setData({
                adWatchStartTime: Date.now()
            }), !wx.createRewardedVideoAd) return wx.showToast({
                title: "当前环境不支持广告",
                icon: "none"
            }), void this.handleCodeError("当前环境不支持广告功能");
            var e = wx.createRewardedVideoAd({
                adUnitId: "adunit-xxxxxxxxxxxxxxxx"
            });
            e.onLoad(function() {}), e.onError(function() {
                wx.showToast({
                    title: "广告加载失败",
                    icon: "none"
                }), t.handleCodeError("广告加载失败，请重试");
            }), e.onClose(function(e) {
                var o = Date.now() - t.data.adWatchStartTime;
                e && e.isEnded ? t.handleAdWatchCompleted(o) : (wx.showToast({
                    title: "请观看完整广告",
                    icon: "none"
                }), t.handleCodeError("需要观看完整广告才能获取验证码"));
            }), e.show().then(function() {}).catch(function() {
                e.load().then(function() {
                    return e.show();
                }).catch(function() {
                    wx.showToast({
                        title: "广告显示失败",
                        icon: "none"
                    }), t.handleCodeError("广告显示失败，请重试");
                });
            });
        } catch (t) {
            this.handleCodeError("广告功能异常，请重试");
        }
    },
    handleCodeError: function(t) {
        this.setData({
            codeLoading: !1,
            codeError: !0,
            verificationCode: "",
            codeCharacters: []
        }), this.showErrorDialog(t);
    },
    showErrorDialog: function(t) {
        var e = "获取失败", o = t, n = !1;
        t.includes("Session not found") || t.includes("Session not verified") || t.includes("会话不存在") || t.includes("会话状态无效") || t.includes("expired") || t.includes("过期") ? (e = "会话已过期", 
        o = "当前会话已过期或无效，需要重新进入小程序获取验证码。", n = !0) : t.includes("Project ID mismatch") || t.includes("项目ID不匹配") ? (e = "项目不匹配", 
        o = "项目信息不匹配，请重新进入小程序。", n = !0) : t.includes("Invalid signature") || t.includes("签名验证失败") ? (e = "验证失败", 
        o = "安全验证失败，请重新进入小程序。", n = !0) : t.includes("Rate limit") || t.includes("频率限制") ? (e = "请求过于频繁", 
        o = "请求频率过高，请稍后再试。") : (t.includes("网络") || t.includes("连接")) && (e = "网络错误", o = "网络连接异常，请检查网络后重试。"), 
        n ? wx.showModal({
            title: e,
            content: o + '\n\n点击"重新进入"将返回首页重新开始。',
            showCancel: !0,
            cancelText: "取消",
            confirmText: "重新进入",
            confirmColor: "#007aff",
            success: function(t) {
                t.confirm && wx.reLaunch({
                    url: "/pages/software-select/software-select"
                });
            }
        }) : wx.showModal({
            title: e,
            content: o,
            showCancel: !1,
            confirmText: "我知道了",
            confirmColor: "#007aff"
        });
    },
    refreshVerificationCode: function() {
        this.clearCountdown();
        var t = this.data.sessionId, e = this.data.tempToken;
        t && e ? this.fetchVerificationCodeWithSession(t, e) : this.fetchVerificationCode();
    },
    startCountdown: function(t) {
        var e = this;
        this.clearCountdown();
        var o = new Date(t).getTime(), n = Date.now();
        if (o <= n) this.setData({
            "countdown.show": !0,
            "countdown.expired": !0,
            "countdown.totalSeconds": 0
        }); else {
            var r = Math.floor((o - n) / 1e3);
            this.setData({
                "countdown.show": !0,
                "countdown.expired": !1,
                "countdown.totalSeconds": r
            }), this.updateCountdownDisplay(r), this.countdownTimer = setInterval(function() {
                var t = Date.now(), n = Math.floor((o - t) / 1e3);
                n <= 0 ? (e.setData({
                    "countdown.expired": !0,
                    "countdown.totalSeconds": 0,
                    "countdown.timeText": ""
                }), e.clearCountdown(), wx.showModal({
                    title: "验证码已过期",
                    content: "当前验证码已过期，请重新获取验证码",
                    showCancel: !1,
                    confirmText: "重新获取",
                    success: function() {
                        e.refreshVerificationCode();
                    }
                })) : (e.setData({
                    "countdown.totalSeconds": n
                }), e.updateCountdownDisplay(n));
            }, 1e3);
        }
    },
    updateCountdownDisplay: function(t) {
        var e = Math.floor(t / 86400), o = Math.floor(t % 86400 / 3600), n = Math.floor(t % 3600 / 60), r = t % 60, a = "";
        e > 0 && (a += "".concat(e, "天")), (o > 0 || e > 0) && (a += "".concat(o.toString().padStart(2, "0"), "时")), 
        a += "".concat(n.toString().padStart(2, "0"), "分").concat(r.toString().padStart(2, "0"), "秒"), 
        this.setData({
            "countdown.timeText": a
        });
    },
    clearCountdown: function() {
        this.countdownTimer && (clearInterval(this.countdownTimer), this.countdownTimer = null);
    },
    copyVerificationCode: function() {
        this.data.verificationCode && wx.setClipboardData({
            data: this.data.verificationCode,
            success: function() {
                wx.showToast({
                    title: "验证码已复制",
                    icon: "success",
                    duration: 1500
                });
            },
            fail: function() {
                wx.showToast({
                    title: "复制失败",
                    icon: "error",
                    duration: 1500
                });
            }
        });
    },
    decryptVerificationDataWithTempKey: function(t, e) {
        try {
            if (!t.startsWith("YAN_TEMP_") || !t.endsWith("_END")) throw new Error("无效的加密数据格式");
            var n = t.slice(9, -4), r = o.enc.Base64.parse(e), a = o.enc.Base64.parse(n), i = o.lib.WordArray.create(a.words.slice(0, 4)), s = o.lib.WordArray.create(a.words.slice(4)), c = o.AES.decrypt({
                ciphertext: s
            }, r, {
                iv: i,
                mode: o.mode.CBC,
                padding: o.pad.Pkcs7
            }).toString(o.enc.Utf8);
            if (!c) throw new Error("解密结果为空");
            return JSON.parse(c);
        } catch (t) {
            throw new Error("解密失败");
        }
    },
    decryptVerificationData: function(t) {
        if (!t.startsWith("YAN_AES_") || !t.endsWith("_END")) throw new Error("数据格式错误");
        var e = t.slice(8, -4);
        try {
            var n = o.SHA256("Yan&!SD#asui*+.Ge"), r = o.enc.Base64.parse(e), a = o.lib.WordArray.create(r.words.slice(0, 4)), i = o.lib.WordArray.create(r.words.slice(4)), s = o.AES.decrypt({
                ciphertext: i
            }, n, {
                iv: a,
                mode: o.mode.CBC,
                padding: o.pad.Pkcs7
            }).toString(o.enc.Utf8);
            if (!s) throw new Error("解密结果为空");
            return JSON.parse(s);
        } catch (t) {
            throw new Error("解密失败: ".concat(t));
        }
    },
    fetchHitokoto: function() {
        var t = this;
        this.setData({
            "hitokoto.loading": !0,
            "hitokoto.error": !1
        });
        var e = [ "d", "b", "i" ], o = e[Math.floor(Math.random() * e.length)];
        wx.request({
            url: "https://v1.hitokoto.cn",
            method: "GET",
            header: {
                "content-type": "application/json; charset=utf-8"
            },
            data: {
                c: o,
                encode: "json",
                max_length: 25
            },
            success: function(e) {
                if (200 === e.statusCode && e.data) {
                    var o = e.data.hitokoto || "暂无内容";
                    (o = String(o)).endsWith("。") && (o = o.slice(0, -1)), o.length > 25 && (o = o.substring(0, 22) + "..."), 
                    t.setData({
                        "hitokoto.text": o,
                        "hitokoto.from": e.data.from || "",
                        "hitokoto.from_who": e.data.from_who || "",
                        "hitokoto.loading": !1,
                        "hitokoto.error": !1
                    });
                } else t.handleHitokotoError();
            },
            fail: function() {
                t.handleHitokotoError();
            }
        });
    },
    handleHitokotoError: function() {
        this.setData({
            "hitokoto.text": "享受今天，期待明天",
            "hitokoto.from": "",
            "hitokoto.from_who": "",
            "hitokoto.loading": !1,
            "hitokoto.error": !0
        });
    },
    startExitAnimation: function() {
        var t = this;
        return new Promise(function(e) {
            t.setData({
                "animationState.isExiting": !0
            }), setTimeout(function() {
                e(!0);
            }, 400);
        });
    },
    navigateBack: function() {
        this.startExitAnimation().then(function() {
            wx.navigateBack();
        });
    },
    onShow: function() {
        this.setData({
            "animationState.isExiting": !1
        });
    },
    onHide: function() {},
    onUnload: function() {
        this.setData({
            "animationState.isEntering": !1,
            "animationState.isExiting": !1,
            "animationState.contentReady": !1
        }), this.clearCountdown();
    }
});