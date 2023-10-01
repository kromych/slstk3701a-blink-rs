#[doc = "Register `TFRMODE` reader"]
pub type R = crate::R<TFRMODE_SPEC>;
#[doc = "Register `TFRMODE` writer"]
pub type W = crate::W<TFRMODE_SPEC>;
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLKCNTEN` reader - Block Count Enable"]
pub type BLKCNTEN_R = crate::BitReader;
#[doc = "Field `BLKCNTEN` writer - Block Count Enable"]
pub type BLKCNTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOCMDEN` reader - Auto Command Enable"]
pub type AUTOCMDEN_R = crate::FieldReader<AUTOCMDEN_A>;
#[doc = "Auto Command Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AUTOCMDEN_A {
    #[doc = "0: Auto CMD Disabled"]
    ACMDDISABLED = 0,
    #[doc = "1: Auto CMD12 Enable"]
    ACMD12EN = 1,
    #[doc = "2: Auto CMD23 Enable"]
    ACMD23EN = 2,
}
impl From<AUTOCMDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: AUTOCMDEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AUTOCMDEN_A {
    type Ux = u8;
}
impl AUTOCMDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AUTOCMDEN_A> {
        match self.bits {
            0 => Some(AUTOCMDEN_A::ACMDDISABLED),
            1 => Some(AUTOCMDEN_A::ACMD12EN),
            2 => Some(AUTOCMDEN_A::ACMD23EN),
            _ => None,
        }
    }
    #[doc = "Auto CMD Disabled"]
    #[inline(always)]
    pub fn is_acmddisabled(&self) -> bool {
        *self == AUTOCMDEN_A::ACMDDISABLED
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn is_acmd12en(&self) -> bool {
        *self == AUTOCMDEN_A::ACMD12EN
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn is_acmd23en(&self) -> bool {
        *self == AUTOCMDEN_A::ACMD23EN
    }
}
#[doc = "Field `AUTOCMDEN` writer - Auto Command Enable"]
pub type AUTOCMDEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, AUTOCMDEN_A>;
impl<'a, REG, const O: u8> AUTOCMDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto CMD Disabled"]
    #[inline(always)]
    pub fn acmddisabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOCMDEN_A::ACMDDISABLED)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn acmd12en(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOCMDEN_A::ACMD12EN)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn acmd23en(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOCMDEN_A::ACMD23EN)
    }
}
#[doc = "Field `DATDIRSEL` reader - Data Transfer Direction Select"]
pub type DATDIRSEL_R = crate::BitReader;
#[doc = "Field `DATDIRSEL` writer - Data Transfer Direction Select"]
pub type DATDIRSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MULTSINGBLKSEL` reader - Multiple or Single Block Data Transfer Selection"]
pub type MULTSINGBLKSEL_R = crate::BitReader;
#[doc = "Field `MULTSINGBLKSEL` writer - Multiple or Single Block Data Transfer Selection"]
pub type MULTSINGBLKSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESPTYPESEL` reader - Response Type Select"]
pub type RESPTYPESEL_R = crate::FieldReader<RESPTYPESEL_A>;
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESPTYPESEL_A {
    #[doc = "0: No RESP"]
    NORESP = 0,
    #[doc = "1: RESP Length 136"]
    RESP136 = 1,
    #[doc = "2: RESP Length 48"]
    RESP48 = 2,
    #[doc = "3: RESP Length 48 check busy after RESP"]
    BUSYAFTRESP = 3,
}
impl From<RESPTYPESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RESPTYPESEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RESPTYPESEL_A {
    type Ux = u8;
}
impl RESPTYPESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESPTYPESEL_A {
        match self.bits {
            0 => RESPTYPESEL_A::NORESP,
            1 => RESPTYPESEL_A::RESP136,
            2 => RESPTYPESEL_A::RESP48,
            3 => RESPTYPESEL_A::BUSYAFTRESP,
            _ => unreachable!(),
        }
    }
    #[doc = "No RESP"]
    #[inline(always)]
    pub fn is_noresp(&self) -> bool {
        *self == RESPTYPESEL_A::NORESP
    }
    #[doc = "RESP Length 136"]
    #[inline(always)]
    pub fn is_resp136(&self) -> bool {
        *self == RESPTYPESEL_A::RESP136
    }
    #[doc = "RESP Length 48"]
    #[inline(always)]
    pub fn is_resp48(&self) -> bool {
        *self == RESPTYPESEL_A::RESP48
    }
    #[doc = "RESP Length 48 check busy after RESP"]
    #[inline(always)]
    pub fn is_busyaftresp(&self) -> bool {
        *self == RESPTYPESEL_A::BUSYAFTRESP
    }
}
#[doc = "Field `RESPTYPESEL` writer - Response Type Select"]
pub type RESPTYPESEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, RESPTYPESEL_A>;
impl<'a, REG, const O: u8> RESPTYPESEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No RESP"]
    #[inline(always)]
    pub fn noresp(self) -> &'a mut crate::W<REG> {
        self.variant(RESPTYPESEL_A::NORESP)
    }
    #[doc = "RESP Length 136"]
    #[inline(always)]
    pub fn resp136(self) -> &'a mut crate::W<REG> {
        self.variant(RESPTYPESEL_A::RESP136)
    }
    #[doc = "RESP Length 48"]
    #[inline(always)]
    pub fn resp48(self) -> &'a mut crate::W<REG> {
        self.variant(RESPTYPESEL_A::RESP48)
    }
    #[doc = "RESP Length 48 check busy after RESP"]
    #[inline(always)]
    pub fn busyaftresp(self) -> &'a mut crate::W<REG> {
        self.variant(RESPTYPESEL_A::BUSYAFTRESP)
    }
}
#[doc = "Field `CMDCRCCHKEN` reader - Command CRC Check Enable"]
pub type CMDCRCCHKEN_R = crate::BitReader;
#[doc = "Field `CMDCRCCHKEN` writer - Command CRC Check Enable"]
pub type CMDCRCCHKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDINDXCHKEN` reader - Command Index Check Enable"]
pub type CMDINDXCHKEN_R = crate::BitReader;
#[doc = "Field `CMDINDXCHKEN` writer - Command Index Check Enable"]
pub type CMDINDXCHKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATPRESSEL` reader - Data Present Select"]
pub type DATPRESSEL_R = crate::BitReader;
#[doc = "Field `DATPRESSEL` writer - Data Present Select"]
pub type DATPRESSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDTYPE` reader - Command Type"]
pub type CMDTYPE_R = crate::FieldReader<CMDTYPE_A>;
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDTYPE_A {
    #[doc = "0: Normal Command"]
    NORMAL = 0,
    #[doc = "1: Suspend command"]
    SUSPEND = 1,
    #[doc = "2: Resume command"]
    RESUME = 2,
    #[doc = "3: Abort command"]
    ABORT = 3,
}
impl From<CMDTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDTYPE_A {
    type Ux = u8;
}
impl CMDTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYPE_A {
        match self.bits {
            0 => CMDTYPE_A::NORMAL,
            1 => CMDTYPE_A::SUSPEND,
            2 => CMDTYPE_A::RESUME,
            3 => CMDTYPE_A::ABORT,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal Command"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CMDTYPE_A::NORMAL
    }
    #[doc = "Suspend command"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMDTYPE_A::SUSPEND
    }
    #[doc = "Resume command"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMDTYPE_A::RESUME
    }
    #[doc = "Abort command"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == CMDTYPE_A::ABORT
    }
}
#[doc = "Field `CMDTYPE` writer - Command Type"]
pub type CMDTYPE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CMDTYPE_A>;
impl<'a, REG, const O: u8> CMDTYPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Command"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTYPE_A::NORMAL)
    }
    #[doc = "Suspend command"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTYPE_A::SUSPEND)
    }
    #[doc = "Resume command"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTYPE_A::RESUME)
    }
    #[doc = "Abort command"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTYPE_A::ABORT)
    }
}
#[doc = "Field `CMDINDEX` reader - Command Index"]
pub type CMDINDEX_R = crate::FieldReader;
#[doc = "Field `CMDINDEX` writer - Command Index"]
pub type CMDINDEX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn blkcnten(&self) -> BLKCNTEN_R {
        BLKCNTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn autocmden(&self) -> AUTOCMDEN_R {
        AUTOCMDEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn datdirsel(&self) -> DATDIRSEL_R {
        DATDIRSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multiple or Single Block Data Transfer Selection"]
    #[inline(always)]
    pub fn multsingblksel(&self) -> MULTSINGBLKSEL_R {
        MULTSINGBLKSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn resptypesel(&self) -> RESPTYPESEL_R {
        RESPTYPESEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdcrcchken(&self) -> CMDCRCCHKEN_R {
        CMDCRCCHKEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdindxchken(&self) -> CMDINDXCHKEN_R {
        CMDINDXCHKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn datpressel(&self) -> DATPRESSEL_R {
        DATPRESSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtype(&self) -> CMDTYPE_R {
        CMDTYPE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<TFRMODE_SPEC, 0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blkcnten(&mut self) -> BLKCNTEN_W<TFRMODE_SPEC, 1> {
        BLKCNTEN_W::new(self)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autocmden(&mut self) -> AUTOCMDEN_W<TFRMODE_SPEC, 2> {
        AUTOCMDEN_W::new(self)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    #[must_use]
    pub fn datdirsel(&mut self) -> DATDIRSEL_W<TFRMODE_SPEC, 4> {
        DATDIRSEL_W::new(self)
    }
    #[doc = "Bit 5 - Multiple or Single Block Data Transfer Selection"]
    #[inline(always)]
    #[must_use]
    pub fn multsingblksel(&mut self) -> MULTSINGBLKSEL_W<TFRMODE_SPEC, 5> {
        MULTSINGBLKSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn resptypesel(&mut self) -> RESPTYPESEL_W<TFRMODE_SPEC, 16> {
        RESPTYPESEL_W::new(self)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcchken(&mut self) -> CMDCRCCHKEN_W<TFRMODE_SPEC, 19> {
        CMDCRCCHKEN_W::new(self)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindxchken(&mut self) -> CMDINDXCHKEN_W<TFRMODE_SPEC, 20> {
        CMDINDXCHKEN_W::new(self)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    #[must_use]
    pub fn datpressel(&mut self) -> DATPRESSEL_W<TFRMODE_SPEC, 21> {
        DATPRESSEL_W::new(self)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtype(&mut self) -> CMDTYPE_W<TFRMODE_SPEC, 22> {
        CMDTYPE_W::new(self)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<TFRMODE_SPEC, 24> {
        CMDINDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transfer Mode and Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfrmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfrmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFRMODE_SPEC;
impl crate::RegisterSpec for TFRMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfrmode::R`](R) reader structure"]
impl crate::Readable for TFRMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tfrmode::W`](W) writer structure"]
impl crate::Writable for TFRMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFRMODE to value 0"]
impl crate::Resettable for TFRMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
