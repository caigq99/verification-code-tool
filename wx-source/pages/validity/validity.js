Page({
    data: {
        loading: !0,
        error: !1,
        errorMessage: "获取失败，请检查网络后重试",
        software: "YAugment",
        softwareInfo: {
            name: "YAugment",
            description: "Y 系列"
        },
        validityInfo: {
            verification: {
                enabled: !1,
                durationHours: 0,
                durationText: "",
                detailText: ""
            },
            vipQq: {
                enabled: !1,
                durationHours: 0,
                durationText: "",
                detailText: ""
            }
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
            contentReady: !1,
            titleReady: !1,
            cardsReady: !1
        },
        systemInfo: {
            backButtonTop: 44,
            backButtonLeft: 16
        }
    },
    onLoad: function(t) {
        var o = t.software || "YAugment";
        this.setSoftwareInfo(o), this.initSystemInfo(), this.initPageAnimation(), this.fetchValidityInfo(), 
        this.fetchHitokoto();
    },
    setSoftwareInfo: function(t) {
        var o = {
            YCursor: {
                name: "YCursor",
                description: "Y 系列"
            },
            YAugment: {
                name: "YAugment",
                description: "Y 系列"
            }
        }, i = o[t] || o.YAugment;
        this.setData({
            software: t,
            softwareInfo: i
        });
    },
    initSystemInfo: function() {
        var t = wx.getSystemInfoSync(), o = wx.getMenuButtonBoundingClientRect(), i = o.top + 6, a = t.windowWidth - o.right + 8;
        this.setData({
            "systemInfo.backButtonTop": i,
            "systemInfo.backButtonLeft": a
        });
    },
    initPageAnimation: function() {
        var t = this;
        wx.nextTick(function() {
            t.setData({
                "animationState.isEntering": !0
            }), setTimeout(function() {
                t.setData({
                    "animationState.titleReady": !0
                });
            }, 100), setTimeout(function() {
                t.setData({
                    "animationState.contentReady": !0
                });
            }, 300), setTimeout(function() {
                t.setData({
                    "animationState.cardsReady": !0,
                    "animationState.isEntering": !1
                });
            }, 500);
        });
    },
    fetchValidityInfo: function() {
        var t = this;
        this.setData({
            loading: !0,
            error: !1
        });
        var o = {
            project_id: this.data.software
        };
        wx.request({
            url: "https://app.yan.vin/api/public/config",
            method: "GET",
            data: o,
            header: {
                "Content-Type": "application/json"
            },
            success: function(o) {
                200 === o.statusCode && o.data ? o.data.error ? t.handleError() : t.processValidityData(o.data) : 429 === o.statusCode ? t.handleRateLimitError() : t.handleError();
            },
            fail: function(o) {
                t.handleError();
            }
        });
    },
    processValidityData: function(t) {
        var o, i, a, e, n = {
            verification: {
                enabled: (null === (o = t.verification) || void 0 === o ? void 0 : o.enabled) || !1,
                durationHours: (null === (i = t.verification) || void 0 === i ? void 0 : i.duration_hours) || 0,
                durationText: "",
                detailText: ""
            },
            vipQq: {
                enabled: (null === (a = t.vip_qq) || void 0 === a ? void 0 : a.enabled) || !1,
                durationHours: (null === (e = t.vip_qq) || void 0 === e ? void 0 : e.duration_hours) || 0,
                durationText: "",
                detailText: ""
            }
        };
        if (n.verification.enabled) {
            var r = this.formatDuration(n.verification.durationHours);
            n.verification.durationText = r.durationText, n.verification.detailText = r.detailText;
        }
        if (n.vipQq.enabled) {
            var s = this.formatDuration(n.vipQq.durationHours);
            n.vipQq.durationText = s.durationText, n.vipQq.detailText = s.detailText;
        }
        this.setData({
            loading: !1,
            error: !1,
            validityInfo: n
        });
    },
    formatDuration: function(t) {
        var o = Math.floor(t / 24), i = t % 24, a = "".concat(t, "小时"), e = "";
        return o > 0 ? (e = "".concat(o, "天"), i > 0 && (e += " ".concat(i.toString().padStart(2, "0"), " 个小时"))) : e = "".concat(t, " 个小时"), 
        {
            durationText: a,
            detailText: e
        };
    },
    handleRateLimitError: function() {
        this.setData({
            loading: !1,
            error: !0,
            errorMessage: "频率过高，请等待10分钟后再试"
        });
    },
    handleError: function() {
        this.setData({
            loading: !1,
            error: !0,
            errorMessage: "获取失败，请检查网络后重试"
        });
    },
    fetchHitokoto: function() {
        var t = this;
        this.setData({
            "hitokoto.loading": !0,
            "hitokoto.error": !1
        });
        var o = [ "d", "b", "i" ], i = o[Math.floor(Math.random() * o.length)];
        wx.request({
            url: "https://v1.hitokoto.cn",
            method: "GET",
            header: {
                "content-type": "application/json; charset=utf-8"
            },
            data: {
                c: i,
                encode: "json",
                max_length: 25
            },
            success: function(o) {
                if (200 === o.statusCode && o.data) {
                    var i = o.data.hitokoto || "暂无内容";
                    (i = String(i)).endsWith("。") && (i = i.slice(0, -1)), i.length > 25 && (i = i.substring(0, 22) + "..."), 
                    t.setData({
                        "hitokoto.text": i,
                        "hitokoto.from": o.data.from || "",
                        "hitokoto.from_who": o.data.from_who || "",
                        "hitokoto.loading": !1,
                        "hitokoto.error": !1
                    });
                } else t.handleHitokotoError();
            },
            fail: function(o) {
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
    navigateBack: function() {
        wx.navigateBack();
    }
});