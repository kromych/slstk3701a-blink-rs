#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE` reader - RTCC Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - RTCC Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DebugrunR = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRECCV0TOP` reader - Pre-counter CCV0 Top Value Enable"]
pub type Preccv0topR = crate::BitReader;
#[doc = "Field `PRECCV0TOP` writer - Pre-counter CCV0 Top Value Enable"]
pub type Preccv0topW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCV1TOP` reader - CCV1 Top Value Enable"]
pub type Ccv1topR = crate::BitReader;
#[doc = "Field `CCV1TOP` writer - CCV1 Top Value Enable"]
pub type Ccv1topW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Counter Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cntpresc {
    #[doc = "0: CLKCNT = LFECLKRTCC/1"]
    Div1 = 0,
    #[doc = "1: CLKCNT = LFECLKRTCC/2"]
    Div2 = 1,
    #[doc = "2: CLKCNT = LFECLKRTCC/4"]
    Div4 = 2,
    #[doc = "3: CLKCNT = LFECLKRTCC/8"]
    Div8 = 3,
    #[doc = "4: CLKCNT = LFECLKRTCC/16"]
    Div16 = 4,
    #[doc = "5: CLKCNT = LFECLKRTCC/32"]
    Div32 = 5,
    #[doc = "6: CLKCNT = LFECLKRTCC/64"]
    Div64 = 6,
    #[doc = "7: CLKCNT = LFECLKRTCC/128"]
    Div128 = 7,
    #[doc = "8: CLKCNT = LFECLKRTCC/256"]
    Div256 = 8,
    #[doc = "9: CLKCNT = LFECLKRTCC/512"]
    Div512 = 9,
    #[doc = "10: CLKCNT = LFECLKRTCC/1024"]
    Div1024 = 10,
    #[doc = "11: CLKCNT = LFECLKRTCC/2048"]
    Div2048 = 11,
    #[doc = "12: CLKCNT = LFECLKRTCC/4096"]
    Div4096 = 12,
    #[doc = "13: CLKCNT = LFECLKRTCC/8192"]
    Div8192 = 13,
    #[doc = "14: CLKCNT = LFECLKRTCC/16384"]
    Div16384 = 14,
    #[doc = "15: CLKCNT = LFECLKRTCC/32768"]
    Div32768 = 15,
}
impl From<Cntpresc> for u8 {
    #[inline(always)]
    fn from(variant: Cntpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cntpresc {
    type Ux = u8;
}
impl crate::IsEnum for Cntpresc {}
#[doc = "Field `CNTPRESC` reader - Counter Prescaler Value"]
pub type CntprescR = crate::FieldReader<Cntpresc>;
impl CntprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntpresc {
        match self.bits {
            0 => Cntpresc::Div1,
            1 => Cntpresc::Div2,
            2 => Cntpresc::Div4,
            3 => Cntpresc::Div8,
            4 => Cntpresc::Div16,
            5 => Cntpresc::Div32,
            6 => Cntpresc::Div64,
            7 => Cntpresc::Div128,
            8 => Cntpresc::Div256,
            9 => Cntpresc::Div512,
            10 => Cntpresc::Div1024,
            11 => Cntpresc::Div2048,
            12 => Cntpresc::Div4096,
            13 => Cntpresc::Div8192,
            14 => Cntpresc::Div16384,
            15 => Cntpresc::Div32768,
            _ => unreachable!(),
        }
    }
    #[doc = "CLKCNT = LFECLKRTCC/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Cntpresc::Div1
    }
    #[doc = "CLKCNT = LFECLKRTCC/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Cntpresc::Div2
    }
    #[doc = "CLKCNT = LFECLKRTCC/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Cntpresc::Div4
    }
    #[doc = "CLKCNT = LFECLKRTCC/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Cntpresc::Div8
    }
    #[doc = "CLKCNT = LFECLKRTCC/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Cntpresc::Div16
    }
    #[doc = "CLKCNT = LFECLKRTCC/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Cntpresc::Div32
    }
    #[doc = "CLKCNT = LFECLKRTCC/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Cntpresc::Div64
    }
    #[doc = "CLKCNT = LFECLKRTCC/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Cntpresc::Div128
    }
    #[doc = "CLKCNT = LFECLKRTCC/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Cntpresc::Div256
    }
    #[doc = "CLKCNT = LFECLKRTCC/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Cntpresc::Div512
    }
    #[doc = "CLKCNT = LFECLKRTCC/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Cntpresc::Div1024
    }
    #[doc = "CLKCNT = LFECLKRTCC/2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == Cntpresc::Div2048
    }
    #[doc = "CLKCNT = LFECLKRTCC/4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == Cntpresc::Div4096
    }
    #[doc = "CLKCNT = LFECLKRTCC/8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == Cntpresc::Div8192
    }
    #[doc = "CLKCNT = LFECLKRTCC/16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == Cntpresc::Div16384
    }
    #[doc = "CLKCNT = LFECLKRTCC/32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == Cntpresc::Div32768
    }
}
#[doc = "Field `CNTPRESC` writer - Counter Prescaler Value"]
pub type CntprescW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cntpresc, crate::Safe>;
impl<'a, REG> CntprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKCNT = LFECLKRTCC/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div1)
    }
    #[doc = "CLKCNT = LFECLKRTCC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div2)
    }
    #[doc = "CLKCNT = LFECLKRTCC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div4)
    }
    #[doc = "CLKCNT = LFECLKRTCC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div8)
    }
    #[doc = "CLKCNT = LFECLKRTCC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div16)
    }
    #[doc = "CLKCNT = LFECLKRTCC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div32)
    }
    #[doc = "CLKCNT = LFECLKRTCC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div64)
    }
    #[doc = "CLKCNT = LFECLKRTCC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div128)
    }
    #[doc = "CLKCNT = LFECLKRTCC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div256)
    }
    #[doc = "CLKCNT = LFECLKRTCC/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div512)
    }
    #[doc = "CLKCNT = LFECLKRTCC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div1024)
    }
    #[doc = "CLKCNT = LFECLKRTCC/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div2048)
    }
    #[doc = "CLKCNT = LFECLKRTCC/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div4096)
    }
    #[doc = "CLKCNT = LFECLKRTCC/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div8192)
    }
    #[doc = "CLKCNT = LFECLKRTCC/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div16384)
    }
    #[doc = "CLKCNT = LFECLKRTCC/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div32768)
    }
}
#[doc = "Field `CNTTICK` reader - Counter Prescaler Mode"]
pub type CnttickR = crate::BitReader;
#[doc = "Field `CNTTICK` writer - Counter Prescaler Mode"]
pub type CnttickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUMODETSEN` reader - Backup Mode Timestamp Enable"]
pub type BumodetsenR = crate::BitReader;
#[doc = "Field `BUMODETSEN` writer - Backup Mode Timestamp Enable"]
pub type BumodetsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCFDETEN` reader - Oscillator Failure Detection Enable"]
pub type OscfdetenR = crate::BitReader;
#[doc = "Field `OSCFDETEN` writer - Oscillator Failure Detection Enable"]
pub type OscfdetenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTMODE` reader - Main Counter Mode"]
pub type CntmodeR = crate::BitReader;
#[doc = "Field `CNTMODE` writer - Main Counter Mode"]
pub type CntmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LYEARCORRDIS` reader - Leap Year Correction Disabled"]
pub type LyearcorrdisR = crate::BitReader;
#[doc = "Field `LYEARCORRDIS` writer - Leap Year Correction Disabled"]
pub type LyearcorrdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTCC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pre-counter CCV0 Top Value Enable"]
    #[inline(always)]
    pub fn preccv0top(&self) -> Preccv0topR {
        Preccv0topR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCV1 Top Value Enable"]
    #[inline(always)]
    pub fn ccv1top(&self) -> Ccv1topR {
        Ccv1topR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Counter Prescaler Value"]
    #[inline(always)]
    pub fn cntpresc(&self) -> CntprescR {
        CntprescR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Counter Prescaler Mode"]
    #[inline(always)]
    pub fn cnttick(&self) -> CnttickR {
        CnttickR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Backup Mode Timestamp Enable"]
    #[inline(always)]
    pub fn bumodetsen(&self) -> BumodetsenR {
        BumodetsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Oscillator Failure Detection Enable"]
    #[inline(always)]
    pub fn oscfdeten(&self) -> OscfdetenR {
        OscfdetenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Main Counter Mode"]
    #[inline(always)]
    pub fn cntmode(&self) -> CntmodeR {
        CntmodeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Leap Year Correction Disabled"]
    #[inline(always)]
    pub fn lyearcorrdis(&self) -> LyearcorrdisR {
        LyearcorrdisR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCC Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<'_, CtrlSpec> {
        DebugrunW::new(self, 2)
    }
    #[doc = "Bit 4 - Pre-counter CCV0 Top Value Enable"]
    #[inline(always)]
    pub fn preccv0top(&mut self) -> Preccv0topW<'_, CtrlSpec> {
        Preccv0topW::new(self, 4)
    }
    #[doc = "Bit 5 - CCV1 Top Value Enable"]
    #[inline(always)]
    pub fn ccv1top(&mut self) -> Ccv1topW<'_, CtrlSpec> {
        Ccv1topW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Counter Prescaler Value"]
    #[inline(always)]
    pub fn cntpresc(&mut self) -> CntprescW<'_, CtrlSpec> {
        CntprescW::new(self, 8)
    }
    #[doc = "Bit 12 - Counter Prescaler Mode"]
    #[inline(always)]
    pub fn cnttick(&mut self) -> CnttickW<'_, CtrlSpec> {
        CnttickW::new(self, 12)
    }
    #[doc = "Bit 14 - Backup Mode Timestamp Enable"]
    #[inline(always)]
    pub fn bumodetsen(&mut self) -> BumodetsenW<'_, CtrlSpec> {
        BumodetsenW::new(self, 14)
    }
    #[doc = "Bit 15 - Oscillator Failure Detection Enable"]
    #[inline(always)]
    pub fn oscfdeten(&mut self) -> OscfdetenW<'_, CtrlSpec> {
        OscfdetenW::new(self, 15)
    }
    #[doc = "Bit 16 - Main Counter Mode"]
    #[inline(always)]
    pub fn cntmode(&mut self) -> CntmodeW<'_, CtrlSpec> {
        CntmodeW::new(self, 16)
    }
    #[doc = "Bit 17 - Leap Year Correction Disabled"]
    #[inline(always)]
    pub fn lyearcorrdis(&mut self) -> LyearcorrdisW<'_, CtrlSpec> {
        LyearcorrdisW::new(self, 17)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
