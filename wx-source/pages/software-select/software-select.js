var e = require("../../@babel/runtime/helpers/regeneratorRuntime"), t = require("../../@babel/runtime/helpers/asyncToGenerator"), n = require("../../AE61E3604B3CC87FC8078B67643A8012.js");

Page({
    data: {
        animationState: {
            isEntering: !0,
            isExiting: !1,
            contentReady: !1
        },
        systemInfo: {
            statusBarHeight: 44,
            menuButtonHeight: 32,
            menuButtonTop: 0,
            backButtonTop: 0,
            backButtonLeft: 16
        }
    },
    rewardedVideoAd: null,
    selectedSoftware: "",
    isAdPlaying: !1,
    adStartTime: 0,
    currentSessionId: "",
    currentTempToken: "",
    adWatchStartTime: 0,
    apiSignatureSecret: "",
    apiKeyVersion: 0,
    apiKeyLastUpdate: 0,
    onLoad: function() {
        this.clearEasterCount(), this.getSystemInfo(), this.initPageAnimation(), this.initRewardedVideoAd(), 
        this.fetchCurrentSignatureKey();
    },
    clearEasterCount: function() {
        try {
            wx.setStorageSync("easterCount", 0);
        } catch (e) {}
    },
    getSystemInfo: function() {
        var e = wx.getSystemInfoSync(), t = wx.getMenuButtonBoundingClientRect(), n = t.top + 6, a = e.windowWidth - t.right + 8;
        this.setData({
            "systemInfo.statusBarHeight": e.statusBarHeight,
            "systemInfo.menuButtonHeight": t.height,
            "systemInfo.menuButtonTop": t.top,
            "systemInfo.backButtonTop": n,
            "systemInfo.backButtonLeft": a
        });
    },
    initPageAnimation: function() {
        var e = this;
        wx.nextTick(function() {
            e.setData({
                "animationState.isEntering": !0
            }), setTimeout(function() {
                e.setData({
                    "animationState.contentReady": !0,
                    "animationState.isEntering": !1
                });
            }, 80);
        });
    },
    initRewardedVideoAd: function() {
        var e = this;
        wx.createRewardedVideoAd && (this.rewardedVideoAd = wx.createRewardedVideoAd({
            adUnitId: "adunit-ac487887561b63a2"
        }), this.rewardedVideoAd.onLoad(function() {}), this.rewardedVideoAd.onError(function() {}), 
        this.rewardedVideoAd.onClose(function(t) {
            e.handleAdClose(t);
        }), this.rewardedVideoAd.load().catch(function() {}));
    },
    handleAdClose: function(e) {
        this.isAdPlaying = !1, this.adStartTime = 0, wx.hideToast(), e && e.isEnded ? this.completeThreeStageVerification() : (this.selectedSoftware = "", 
        this.currentSessionId = "", this.currentTempToken = "", wx.showModal({
            title: "视频未观看完整",
            content: "需要观看完整视频广告才能获得验证码，请重新尝试",
            showCancel: !1,
            confirmText: "知道了"
        }));
    },
    completeThreeStageVerification: function() {
        var n = this;
        return t(e().mark(function t() {
            var a;
            return e().wrap(function(e) {
                for (;;) switch (e.prev = e.next) {
                  case 0:
                    return e.prev = 0, wx.showLoading({
                        title: "验证中...",
                        mask: !0
                    }), a = Date.now() - n.adWatchStartTime, e.next = 5, n.verifyAdCompletion(n.currentSessionId, a);

                  case 5:
                    wx.hideLoading(), wx.showToast({
                        title: "验证成功！",
                        icon: "success",
                        duration: 1500
                    }), setTimeout(function() {
                        n.startExitAnimation().then(function() {
                            wx.navigateTo({
                                url: "/pages/code/code?software=".concat(n.selectedSoftware, "&sessionId=").concat(n.currentSessionId, "&tempToken=").concat(encodeURIComponent(n.currentTempToken))
                            }), n.selectedSoftware = "", n.currentSessionId = "", n.currentTempToken = "", n.adWatchStartTime = 0;
                        });
                    }, 1500), e.next = 18;
                    break;

                  case 10:
                    e.prev = 10, e.t0 = e.catch(0), wx.hideLoading(), "频率限制" === e.t0.message || wx.showModal({
                        title: "验证失败",
                        content: "广告验证失败，请重试",
                        showCancel: !1,
                        confirmText: "知道了"
                    }), n.selectedSoftware = "", n.currentSessionId = "", n.currentTempToken = "", n.adWatchStartTime = 0;

                  case 18:
                  case "end":
                    return e.stop();
                }
            }, t, null, [ [ 0, 10 ] ]);
        }))();
    },
    selectSoftware: function(e) {
        var t = this, n = e.currentTarget.dataset.software;
        this.selectedSoftware = n, wx.showModal({
            title: "确认选择",
            content: "您选择了 ".concat(n, "，观看完整视频广告后即可获得验证码，是否继续？"),
            confirmText: "观看视频",
            cancelText: "取消",
            success: function(e) {
                e.confirm ? t.startThreeStageVerification() : t.selectedSoftware = "";
            }
        });
    },
    startThreeStageVerification: function() {
        var n = this;
        return t(e().mark(function t() {
            return e().wrap(function(e) {
                for (;;) switch (e.prev = e.next) {
                  case 0:
                    return e.prev = 0, wx.showLoading({
                        title: "准备验证...",
                        mask: !0
                    }), e.next = 4, n.createAdSession(n.selectedSoftware);

                  case 4:
                    wx.hideLoading(), n.adWatchStartTime = Date.now(), n.playRewardedVideoAd(), e.next = 14;
                    break;

                  case 9:
                    e.prev = 9, e.t0 = e.catch(0), wx.hideLoading(), "频率限制" === e.t0.message || wx.showModal({
                        title: "获取验证码流程启动失败",
                        content: "无法启动获取流程，请检查网络或查看文档后再进行重试",
                        showCancel: !1,
                        confirmText: "知道了"
                    }), n.selectedSoftware = "";

                  case 14:
                  case "end":
                    return e.stop();
                }
            }, t, null, [ [ 0, 9 ] ]);
        }))();
    },
    playRewardedVideoAd: function() {
        var e = this;
        this.isAdPlaying ? wx.showToast({
            title: "广告正在加载中，请稍候...",
            icon: "none",
            duration: 1500
        }) : (this.isAdPlaying = !0, this.adStartTime = Date.now(), wx.showToast({
            title: "正在加载广告...",
            icon: "loading",
            duration: 15e3
        }), this.rewardedVideoAd ? this.rewardedVideoAd.show().then(function() {
            wx.hideToast();
        }).catch(function() {
            var t;
            wx.showToast({
                title: "正在重新加载广告...",
                icon: "loading",
                duration: 5e3
            }), null === (t = e.rewardedVideoAd) || void 0 === t || t.load().then(function() {
                var t;
                return null === (t = e.rewardedVideoAd) || void 0 === t ? void 0 : t.show();
            }).then(function() {
                wx.hideToast();
            }).catch(function() {
                wx.hideToast(), wx.showToast({
                    title: "广告加载失败，请稍后重试或查看文档说明解决",
                    icon: "none",
                    duration: 2e3
                }), e.selectedSoftware = "", e.isAdPlaying = !1;
            });
        }) : (wx.hideToast(), this.startExitAnimation().then(function() {
            wx.navigateTo({
                url: "/pages/code/code?software=".concat(e.selectedSoftware)
            }), e.selectedSoftware = "", e.isAdPlaying = !1;
        })));
    },
    startExitAnimation: function() {
        var e = this;
        return new Promise(function(t) {
            e.setData({
                "animationState.isExiting": !0
            }), setTimeout(function() {
                t(!0);
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
        }), this.rewardedVideoAd && this.rewardedVideoAd.load().catch(function() {});
    },
    onHide: function() {
        this.isAdPlaying && this.adStartTime > 0 && (Date.now() - this.adStartTime < 5e3 && wx.hideToast());
    },
    onUnload: function() {
        this.setData({
            "animationState.isEntering": !1,
            "animationState.isExiting": !1,
            "animationState.contentReady": !1
        }), this.rewardedVideoAd && (this.rewardedVideoAd.destroy && this.rewardedVideoAd.destroy(), 
        this.rewardedVideoAd = null);
    },
    fetchCurrentSignatureKey: function() {
        var e = this;
        return new Promise(function(t, n) {
            var a = Date.now();
            e.apiSignatureSecret && a - e.apiKeyLastUpdate < 3e5 ? t() : wx.request({
                url: "https://app.yan.vin/api/public/signature-key",
                method: "GET",
                header: {
                    "Content-Type": "application/json",
                    Referer: "https://servicewechat.com/wx421aabd7feefa0ed/devtools/page-frame.html"
                },
                success: function(n) {
                    200 === n.statusCode && n.data ? n.data.key && n.data.version ? (e.apiSignatureSecret = n.data.key, 
                    e.apiKeyVersion = n.data.version, e.apiKeyLastUpdate = a, t()) : (console.error("获取签名密钥失败: 响应数据格式错误"), 
                    e.apiSignatureSecret = "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210", 
                    e.apiKeyVersion = 1, e.apiKeyLastUpdate = a, t()) : (console.error("获取签名密钥失败: HTTP状态码", n.statusCode), 
                    e.apiSignatureSecret = "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210", 
                    e.apiKeyVersion = 1, e.apiKeyLastUpdate = a, t());
                },
                fail: function(n) {
                    console.error("获取签名密钥网络请求失败:", n), e.apiSignatureSecret = "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210", 
                    e.apiKeyVersion = 1, e.apiKeyLastUpdate = a, t();
                }
            });
        });
    },
    generateApiSignature: function(e, t, a, i, r) {
        try {
            this.apiSignatureSecret || (console.warn("API签名密钥未初始化，使用默认v1密钥"), this.apiSignatureSecret = "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210", 
            this.apiKeyVersion = 1, this.apiKeyLastUpdate = Date.now());
            var o = "";
            if (a) {
                var s = Object.keys(a).sort(), d = {};
                s.forEach(function(e) {
                    d[e] = a[e];
                }), o = JSON.stringify(d).replace(/\s/g, "");
            }
            var c = "".concat(e.toUpperCase(), "\n").concat(t, "\n").concat(o, "\n").concat(i, "\n").concat(r);
            return n.HmacSHA256(c, this.apiSignatureSecret).toString();
        } catch (e) {
            return null;
        }
    },
    generateSignedHeaders: function(e, t, n) {
        var a = Math.floor(Date.now() / 1e3).toString(), i = Math.random().toString(36).substr(2, 16);
        return {
            "Content-Type": "application/json",
            "X-YAN-Signature": this.generateApiSignature(e, t, n, a, i),
            "X-YAN-Timestamp": a,
            "X-YAN-Nonce": i
        };
    },
    generateDeviceFingerprint: function() {
        try {
            for (var e = wx.getSystemInfoSync(), t = {
                model: e.model || "unknown",
                platform: e.platform || "unknown",
                version: e.version || "unknown",
                system: e.system || "unknown",
                language: e.language || "zh_CN",
                screenWidth: e.screenWidth || 0,
                screenHeight: e.screenHeight || 0,
                pixelRatio: e.pixelRatio || 1,
                brand: e.brand || "unknown",
                timestamp: Date.now(),
                random: Math.random()
            }, n = JSON.stringify(t), a = 0, i = 0; i < n.length; i++) {
                a = (a << 5) - a + n.charCodeAt(i), a &= a;
            }
            return Math.abs(a).toString(16).padStart(8, "0") + Date.now().toString(16) + Math.random().toString(16).substr(2, 8);
        } catch (e) {
            return "fallback_" + Date.now().toString(16) + "_" + Math.random().toString(16).substr(2, 8);
        }
    },
    createAdSession: function(n) {
        var a = this;
        return t(e().mark(function t() {
            return e().wrap(function(e) {
                for (;;) switch (e.prev = e.next) {
                  case 0:
                    return e.abrupt("return", new Promise(function(e, t) {
                        var i = a.generateDeviceFingerprint(), r = {
                            project_id: n,
                            device_fingerprint: i
                        }, o = a.generateSignedHeaders("POST", "/api/public/ad-session", r);
                        wx.request({
                            url: "https://app.yan.vin/api/public/ad-session",
                            method: "POST",
                            header: o,
                            data: r,
                            success: function(n) {
                                200 === n.statusCode && n.data ? n.data.error ? t(new Error(n.data.error)) : (a.currentSessionId = n.data.session_id, 
                                e(n.data)) : 429 === n.statusCode ? (wx.showModal({
                                    title: "请求频率过高",
                                    content: "频率太高，无法启动获取流程，请等待一会稍后再试。请不要短时间内频繁重复，避免进风控小黑屋。",
                                    showCancel: !1,
                                    confirmText: "我知道了",
                                    confirmColor: "#ff6b6b"
                                }), t(new Error("频率限制"))) : t(new Error("网络请求失败"));
                            },
                            fail: function() {
                                t(new Error("网络连接失败"));
                            }
                        });
                    }));

                  case 1:
                  case "end":
                    return e.stop();
                }
            }, t);
        }))();
    },
    verifyAdCompletion: function(n, a) {
        var i = this;
        return t(e().mark(function t() {
            return e().wrap(function(e) {
                for (;;) switch (e.prev = e.next) {
                  case 0:
                    return e.abrupt("return", new Promise(function(e, t) {
                        var r = {
                            session_id: n,
                            watch_duration: a,
                            completion_proof: "miniprogram_ad_completed"
                        }, o = i.generateSignedHeaders("POST", "/api/public/ad-verify", r);
                        wx.request({
                            url: "https://app.yan.vin/api/public/ad-verify",
                            method: "POST",
                            header: o,
                            data: r,
                            success: function(n) {
                                200 === n.statusCode && n.data ? n.data.error ? t(new Error(n.data.error)) : (i.currentTempToken = n.data.temp_token, 
                                e(n.data)) : 429 === n.statusCode ? (wx.showModal({
                                    title: "请求频率过高",
                                    content: "频率太高，无法完成验证流程，请等待一会稍后再试。请不要短时间内频繁重复，避免进风控小黑屋。",
                                    showCancel: !1,
                                    confirmText: "我知道了",
                                    confirmColor: "#ff6b6b"
                                }), t(new Error("频率限制"))) : t(new Error("网络请求失败"));
                            },
                            fail: function() {
                                t(new Error("网络连接失败"));
                            }
                        });
                    }));

                  case 1:
                  case "end":
                    return e.stop();
                }
            }, t);
        }))();
    }
});