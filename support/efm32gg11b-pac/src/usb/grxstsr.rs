#[doc = "Register `GRXSTSR` reader"]
pub type R = crate::R<GRXSTSR_SPEC>;
#[doc = "Field `CHNUM` reader - Channel Number"]
pub type CHNUM_R = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte Count"]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader<DPID_A>;
#[doc = "Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DPID_A {
    #[doc = "0: DATA0 PID."]
    DATA0 = 0,
    #[doc = "1: DATA1 PID."]
    DATA1 = 1,
    #[doc = "2: DATA2 PID."]
    DATA2 = 2,
    #[doc = "3: MDATA PID."]
    MDATA = 3,
}
impl From<DPID_A> for u8 {
    #[inline(always)]
    fn from(variant: DPID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DPID_A {
    type Ux = u8;
}
impl DPID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DPID_A {
        match self.bits {
            0 => DPID_A::DATA0,
            1 => DPID_A::DATA1,
            2 => DPID_A::DATA2,
            3 => DPID_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0 PID."]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DPID_A::DATA0
    }
    #[doc = "DATA1 PID."]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DPID_A::DATA1
    }
    #[doc = "DATA2 PID."]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == DPID_A::DATA2
    }
    #[doc = "MDATA PID."]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == DPID_A::MDATA
    }
}
#[doc = "Field `PKTSTS` reader - Packet Status"]
pub type PKTSTS_R = crate::FieldReader<PKTSTS_A>;
#[doc = "Packet Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PKTSTS_A {
    #[doc = "1: Device mode: Global OUT NAK (triggers an interrupt)."]
    GOUTNAK = 1,
    #[doc = "2: Host mode: IN data packet received. Device mode: OUT data packet received."]
    PKTRCV = 2,
    #[doc = "3: Host mode: IN transfer completed (triggers an interrupt). Device mode: OUT transfer completed (triggers an interrupt)."]
    XFERCOMPL = 3,
    #[doc = "4: Device mode: SETUP transaction completed (triggers an interrupt)."]
    SETUPCOMPL = 4,
    #[doc = "5: Host mode: Data toggle error (triggers an interrupt)."]
    TGLERR = 5,
    #[doc = "6: Device mode: SETUP data packet received."]
    SETUPRCV = 6,
    #[doc = "7: Host mode: Channel halted (triggers an interrupt)."]
    CHLT = 7,
}
impl From<PKTSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: PKTSTS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PKTSTS_A {
    type Ux = u8;
}
impl PKTSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PKTSTS_A> {
        match self.bits {
            1 => Some(PKTSTS_A::GOUTNAK),
            2 => Some(PKTSTS_A::PKTRCV),
            3 => Some(PKTSTS_A::XFERCOMPL),
            4 => Some(PKTSTS_A::SETUPCOMPL),
            5 => Some(PKTSTS_A::TGLERR),
            6 => Some(PKTSTS_A::SETUPRCV),
            7 => Some(PKTSTS_A::CHLT),
            _ => None,
        }
    }
    #[doc = "Device mode: Global OUT NAK (triggers an interrupt)."]
    #[inline(always)]
    pub fn is_goutnak(&self) -> bool {
        *self == PKTSTS_A::GOUTNAK
    }
    #[doc = "Host mode: IN data packet received. Device mode: OUT data packet received."]
    #[inline(always)]
    pub fn is_pktrcv(&self) -> bool {
        *self == PKTSTS_A::PKTRCV
    }
    #[doc = "Host mode: IN transfer completed (triggers an interrupt). Device mode: OUT transfer completed (triggers an interrupt)."]
    #[inline(always)]
    pub fn is_xfercompl(&self) -> bool {
        *self == PKTSTS_A::XFERCOMPL
    }
    #[doc = "Device mode: SETUP transaction completed (triggers an interrupt)."]
    #[inline(always)]
    pub fn is_setupcompl(&self) -> bool {
        *self == PKTSTS_A::SETUPCOMPL
    }
    #[doc = "Host mode: Data toggle error (triggers an interrupt)."]
    #[inline(always)]
    pub fn is_tglerr(&self) -> bool {
        *self == PKTSTS_A::TGLERR
    }
    #[doc = "Device mode: SETUP data packet received."]
    #[inline(always)]
    pub fn is_setuprcv(&self) -> bool {
        *self == PKTSTS_A::SETUPRCV
    }
    #[doc = "Host mode: Channel halted (triggers an interrupt)."]
    #[inline(always)]
    pub fn is_chlt(&self) -> bool {
        *self == PKTSTS_A::CHLT
    }
}
#[doc = "Field `FN` reader - Frame Number"]
pub type FN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Channel Number"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet Status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[doc = "Receive Status Debug Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRXSTSR_SPEC;
impl crate::RegisterSpec for GRXSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsr::R`](R) reader structure"]
impl crate::Readable for GRXSTSR_SPEC {}
#[doc = "`reset()` method sets GRXSTSR to value 0"]
impl crate::Resettable for GRXSTSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
