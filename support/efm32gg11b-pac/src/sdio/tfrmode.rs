#[doc = "Register `TFRMODE` reader"]
pub type R = crate::R<TfrmodeSpec>;
#[doc = "Register `TFRMODE` writer"]
pub type W = crate::W<TfrmodeSpec>;
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKCNTEN` reader - Block Count Enable"]
pub type BlkcntenR = crate::BitReader;
#[doc = "Field `BLKCNTEN` writer - Block Count Enable"]
pub type BlkcntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Auto Command Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Autocmden {
    #[doc = "0: Auto CMD Disabled"]
    Acmddisabled = 0,
    #[doc = "1: Auto CMD12 Enable"]
    Acmd12en = 1,
    #[doc = "2: Auto CMD23 Enable"]
    Acmd23en = 2,
}
impl From<Autocmden> for u8 {
    #[inline(always)]
    fn from(variant: Autocmden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Autocmden {
    type Ux = u8;
}
impl crate::IsEnum for Autocmden {}
#[doc = "Field `AUTOCMDEN` reader - Auto Command Enable"]
pub type AutocmdenR = crate::FieldReader<Autocmden>;
impl AutocmdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Autocmden> {
        match self.bits {
            0 => Some(Autocmden::Acmddisabled),
            1 => Some(Autocmden::Acmd12en),
            2 => Some(Autocmden::Acmd23en),
            _ => None,
        }
    }
    #[doc = "Auto CMD Disabled"]
    #[inline(always)]
    pub fn is_acmddisabled(&self) -> bool {
        *self == Autocmden::Acmddisabled
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn is_acmd12en(&self) -> bool {
        *self == Autocmden::Acmd12en
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn is_acmd23en(&self) -> bool {
        *self == Autocmden::Acmd23en
    }
}
#[doc = "Field `AUTOCMDEN` writer - Auto Command Enable"]
pub type AutocmdenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Autocmden>;
impl<'a, REG> AutocmdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto CMD Disabled"]
    #[inline(always)]
    pub fn acmddisabled(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmden::Acmddisabled)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn acmd12en(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmden::Acmd12en)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn acmd23en(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmden::Acmd23en)
    }
}
#[doc = "Field `DATDIRSEL` reader - Data Transfer Direction Select"]
pub type DatdirselR = crate::BitReader;
#[doc = "Field `DATDIRSEL` writer - Data Transfer Direction Select"]
pub type DatdirselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTSINGBLKSEL` reader - Multiple or Single Block Data Transfer Selection"]
pub type MultsingblkselR = crate::BitReader;
#[doc = "Field `MULTSINGBLKSEL` writer - Multiple or Single Block Data Transfer Selection"]
pub type MultsingblkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resptypesel {
    #[doc = "0: No RESP"]
    Noresp = 0,
    #[doc = "1: RESP Length 136"]
    Resp136 = 1,
    #[doc = "2: RESP Length 48"]
    Resp48 = 2,
    #[doc = "3: RESP Length 48 check busy after RESP"]
    Busyaftresp = 3,
}
impl From<Resptypesel> for u8 {
    #[inline(always)]
    fn from(variant: Resptypesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resptypesel {
    type Ux = u8;
}
impl crate::IsEnum for Resptypesel {}
#[doc = "Field `RESPTYPESEL` reader - Response Type Select"]
pub type ResptypeselR = crate::FieldReader<Resptypesel>;
impl ResptypeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resptypesel {
        match self.bits {
            0 => Resptypesel::Noresp,
            1 => Resptypesel::Resp136,
            2 => Resptypesel::Resp48,
            3 => Resptypesel::Busyaftresp,
            _ => unreachable!(),
        }
    }
    #[doc = "No RESP"]
    #[inline(always)]
    pub fn is_noresp(&self) -> bool {
        *self == Resptypesel::Noresp
    }
    #[doc = "RESP Length 136"]
    #[inline(always)]
    pub fn is_resp136(&self) -> bool {
        *self == Resptypesel::Resp136
    }
    #[doc = "RESP Length 48"]
    #[inline(always)]
    pub fn is_resp48(&self) -> bool {
        *self == Resptypesel::Resp48
    }
    #[doc = "RESP Length 48 check busy after RESP"]
    #[inline(always)]
    pub fn is_busyaftresp(&self) -> bool {
        *self == Resptypesel::Busyaftresp
    }
}
#[doc = "Field `RESPTYPESEL` writer - Response Type Select"]
pub type ResptypeselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resptypesel, crate::Safe>;
impl<'a, REG> ResptypeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No RESP"]
    #[inline(always)]
    pub fn noresp(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::Noresp)
    }
    #[doc = "RESP Length 136"]
    #[inline(always)]
    pub fn resp136(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::Resp136)
    }
    #[doc = "RESP Length 48"]
    #[inline(always)]
    pub fn resp48(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::Resp48)
    }
    #[doc = "RESP Length 48 check busy after RESP"]
    #[inline(always)]
    pub fn busyaftresp(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::Busyaftresp)
    }
}
#[doc = "Field `CMDCRCCHKEN` reader - Command CRC Check Enable"]
pub type CmdcrcchkenR = crate::BitReader;
#[doc = "Field `CMDCRCCHKEN` writer - Command CRC Check Enable"]
pub type CmdcrcchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDINDXCHKEN` reader - Command Index Check Enable"]
pub type CmdindxchkenR = crate::BitReader;
#[doc = "Field `CMDINDXCHKEN` writer - Command Index Check Enable"]
pub type CmdindxchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATPRESSEL` reader - Data Present Select"]
pub type DatpresselR = crate::BitReader;
#[doc = "Field `DATPRESSEL` writer - Data Present Select"]
pub type DatpresselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdtype {
    #[doc = "0: Normal Command"]
    Normal = 0,
    #[doc = "1: Suspend command"]
    Suspend = 1,
    #[doc = "2: Resume command"]
    Resume = 2,
    #[doc = "3: Abort command"]
    Abort = 3,
}
impl From<Cmdtype> for u8 {
    #[inline(always)]
    fn from(variant: Cmdtype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdtype {
    type Ux = u8;
}
impl crate::IsEnum for Cmdtype {}
#[doc = "Field `CMDTYPE` reader - Command Type"]
pub type CmdtypeR = crate::FieldReader<Cmdtype>;
impl CmdtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdtype {
        match self.bits {
            0 => Cmdtype::Normal,
            1 => Cmdtype::Suspend,
            2 => Cmdtype::Resume,
            3 => Cmdtype::Abort,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal Command"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Cmdtype::Normal
    }
    #[doc = "Suspend command"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Cmdtype::Suspend
    }
    #[doc = "Resume command"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Cmdtype::Resume
    }
    #[doc = "Abort command"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == Cmdtype::Abort
    }
}
#[doc = "Field `CMDTYPE` writer - Command Type"]
pub type CmdtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmdtype, crate::Safe>;
impl<'a, REG> CmdtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Command"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Normal)
    }
    #[doc = "Suspend command"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Suspend)
    }
    #[doc = "Resume command"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Resume)
    }
    #[doc = "Abort command"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Abort)
    }
}
#[doc = "Field `CMDINDEX` reader - Command Index"]
pub type CmdindexR = crate::FieldReader;
#[doc = "Field `CMDINDEX` writer - Command Index"]
pub type CmdindexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn blkcnten(&self) -> BlkcntenR {
        BlkcntenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn autocmden(&self) -> AutocmdenR {
        AutocmdenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn datdirsel(&self) -> DatdirselR {
        DatdirselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multiple or Single Block Data Transfer Selection"]
    #[inline(always)]
    pub fn multsingblksel(&self) -> MultsingblkselR {
        MultsingblkselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn resptypesel(&self) -> ResptypeselR {
        ResptypeselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdcrcchken(&self) -> CmdcrcchkenR {
        CmdcrcchkenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdindxchken(&self) -> CmdindxchkenR {
        CmdindxchkenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn datpressel(&self) -> DatpresselR {
        DatpresselR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtype(&self) -> CmdtypeR {
        CmdtypeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CmdindexR {
        CmdindexR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, TfrmodeSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn blkcnten(&mut self) -> BlkcntenW<'_, TfrmodeSpec> {
        BlkcntenW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn autocmden(&mut self) -> AutocmdenW<'_, TfrmodeSpec> {
        AutocmdenW::new(self, 2)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn datdirsel(&mut self) -> DatdirselW<'_, TfrmodeSpec> {
        DatdirselW::new(self, 4)
    }
    #[doc = "Bit 5 - Multiple or Single Block Data Transfer Selection"]
    #[inline(always)]
    pub fn multsingblksel(&mut self) -> MultsingblkselW<'_, TfrmodeSpec> {
        MultsingblkselW::new(self, 5)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn resptypesel(&mut self) -> ResptypeselW<'_, TfrmodeSpec> {
        ResptypeselW::new(self, 16)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdcrcchken(&mut self) -> CmdcrcchkenW<'_, TfrmodeSpec> {
        CmdcrcchkenW::new(self, 19)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdindxchken(&mut self) -> CmdindxchkenW<'_, TfrmodeSpec> {
        CmdindxchkenW::new(self, 20)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn datpressel(&mut self) -> DatpresselW<'_, TfrmodeSpec> {
        DatpresselW::new(self, 21)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtype(&mut self) -> CmdtypeW<'_, TfrmodeSpec> {
        CmdtypeW::new(self, 22)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CmdindexW<'_, TfrmodeSpec> {
        CmdindexW::new(self, 24)
    }
}
#[doc = "Transfer Mode and Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfrmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfrmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfrmodeSpec;
impl crate::RegisterSpec for TfrmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfrmode::R`](R) reader structure"]
impl crate::Readable for TfrmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`tfrmode::W`](W) writer structure"]
impl crate::Writable for TfrmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFRMODE to value 0"]
impl crate::Resettable for TfrmodeSpec {}
