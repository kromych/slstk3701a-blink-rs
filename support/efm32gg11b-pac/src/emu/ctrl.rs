#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EM2BLOCK` reader - Energy Mode 2 Block"]
pub type Em2blockR = crate::BitReader;
#[doc = "Field `EM2BLOCK` writer - Energy Mode 2 Block"]
pub type Em2blockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM2BODDIS` reader - Disable BOD in EM2"]
pub type Em2boddisR = crate::BitReader;
#[doc = "Field `EM2BODDIS` writer - Disable BOD in EM2"]
pub type Em2boddisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM01LD` reader - Reserved for internal use. Do not change."]
pub type Em01ldR = crate::BitReader;
#[doc = "Field `EM01LD` writer - Reserved for internal use. Do not change."]
pub type Em01ldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23VSCALEAUTOWSEN` reader - Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
pub type Em23vscaleautowsenR = crate::BitReader;
#[doc = "Field `EM23VSCALEAUTOWSEN` writer - Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
pub type Em23vscaleautowsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "EM23 Voltage Scale\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em23vscale {
    #[doc = "0: Voltage Scale Level 2"]
    Vscale2 = 0,
    #[doc = "2: Voltage Scale Level 0"]
    Vscale0 = 2,
    #[doc = "3: RESV"]
    Resv = 3,
}
impl From<Em23vscale> for u8 {
    #[inline(always)]
    fn from(variant: Em23vscale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em23vscale {
    type Ux = u8;
}
impl crate::IsEnum for Em23vscale {}
#[doc = "Field `EM23VSCALE` reader - EM23 Voltage Scale"]
pub type Em23vscaleR = crate::FieldReader<Em23vscale>;
impl Em23vscaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Em23vscale> {
        match self.bits {
            0 => Some(Em23vscale::Vscale2),
            2 => Some(Em23vscale::Vscale0),
            3 => Some(Em23vscale::Resv),
            _ => None,
        }
    }
    #[doc = "Voltage Scale Level 2"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == Em23vscale::Vscale2
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == Em23vscale::Vscale0
    }
    #[doc = "RESV"]
    #[inline(always)]
    pub fn is_resv(&self) -> bool {
        *self == Em23vscale::Resv
    }
}
#[doc = "Field `EM23VSCALE` writer - EM23 Voltage Scale"]
pub type Em23vscaleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Em23vscale>;
impl<'a, REG> Em23vscaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Voltage Scale Level 2"]
    #[inline(always)]
    pub fn vscale2(self) -> &'a mut crate::W<REG> {
        self.variant(Em23vscale::Vscale2)
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline(always)]
    pub fn vscale0(self) -> &'a mut crate::W<REG> {
        self.variant(Em23vscale::Vscale0)
    }
    #[doc = "RESV"]
    #[inline(always)]
    pub fn resv(self) -> &'a mut crate::W<REG> {
        self.variant(Em23vscale::Resv)
    }
}
#[doc = "EM4H Voltage Scale\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em4hvscale {
    #[doc = "0: Voltage Scale Level 2"]
    Vscale2 = 0,
    #[doc = "2: Voltage Scale Level 0"]
    Vscale0 = 2,
    #[doc = "3: RESV"]
    Resv = 3,
}
impl From<Em4hvscale> for u8 {
    #[inline(always)]
    fn from(variant: Em4hvscale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em4hvscale {
    type Ux = u8;
}
impl crate::IsEnum for Em4hvscale {}
#[doc = "Field `EM4HVSCALE` reader - EM4H Voltage Scale"]
pub type Em4hvscaleR = crate::FieldReader<Em4hvscale>;
impl Em4hvscaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Em4hvscale> {
        match self.bits {
            0 => Some(Em4hvscale::Vscale2),
            2 => Some(Em4hvscale::Vscale0),
            3 => Some(Em4hvscale::Resv),
            _ => None,
        }
    }
    #[doc = "Voltage Scale Level 2"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == Em4hvscale::Vscale2
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == Em4hvscale::Vscale0
    }
    #[doc = "RESV"]
    #[inline(always)]
    pub fn is_resv(&self) -> bool {
        *self == Em4hvscale::Resv
    }
}
#[doc = "Field `EM4HVSCALE` writer - EM4H Voltage Scale"]
pub type Em4hvscaleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Em4hvscale>;
impl<'a, REG> Em4hvscaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Voltage Scale Level 2"]
    #[inline(always)]
    pub fn vscale2(self) -> &'a mut crate::W<REG> {
        self.variant(Em4hvscale::Vscale2)
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline(always)]
    pub fn vscale0(self) -> &'a mut crate::W<REG> {
        self.variant(Em4hvscale::Vscale0)
    }
    #[doc = "RESV"]
    #[inline(always)]
    pub fn resv(self) -> &'a mut crate::W<REG> {
        self.variant(Em4hvscale::Resv)
    }
}
impl R {
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&self) -> Em2blockR {
        Em2blockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable BOD in EM2"]
    #[inline(always)]
    pub fn em2boddis(&self) -> Em2boddisR {
        Em2boddisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn em01ld(&self) -> Em01ldR {
        Em01ldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
    #[inline(always)]
    pub fn em23vscaleautowsen(&self) -> Em23vscaleautowsenR {
        Em23vscaleautowsenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - EM23 Voltage Scale"]
    #[inline(always)]
    pub fn em23vscale(&self) -> Em23vscaleR {
        Em23vscaleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - EM4H Voltage Scale"]
    #[inline(always)]
    pub fn em4hvscale(&self) -> Em4hvscaleR {
        Em4hvscaleR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&mut self) -> Em2blockW<'_, CtrlSpec> {
        Em2blockW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable BOD in EM2"]
    #[inline(always)]
    pub fn em2boddis(&mut self) -> Em2boddisW<'_, CtrlSpec> {
        Em2boddisW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn em01ld(&mut self) -> Em01ldW<'_, CtrlSpec> {
        Em01ldW::new(self, 3)
    }
    #[doc = "Bit 4 - Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
    #[inline(always)]
    pub fn em23vscaleautowsen(&mut self) -> Em23vscaleautowsenW<'_, CtrlSpec> {
        Em23vscaleautowsenW::new(self, 4)
    }
    #[doc = "Bits 8:9 - EM23 Voltage Scale"]
    #[inline(always)]
    pub fn em23vscale(&mut self) -> Em23vscaleW<'_, CtrlSpec> {
        Em23vscaleW::new(self, 8)
    }
    #[doc = "Bits 16:17 - EM4H Voltage Scale"]
    #[inline(always)]
    pub fn em4hvscale(&mut self) -> Em4hvscaleW<'_, CtrlSpec> {
        Em4hvscaleW::new(self, 16)
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
