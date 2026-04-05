#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - LCD Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - LCD Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Update Data Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Udctrl {
    #[doc = "0: The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    Regular = 0,
    #[doc = "1: The data transfer is done at the next event triggered by the Frame Counter"]
    Fcevent = 1,
    #[doc = "2: The data transfer is done continuously at every LCD frame start"]
    Framestart = 2,
}
impl From<Udctrl> for u8 {
    #[inline(always)]
    fn from(variant: Udctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Udctrl {
    type Ux = u8;
}
impl crate::IsEnum for Udctrl {}
#[doc = "Field `UDCTRL` reader - Update Data Control"]
pub type UdctrlR = crate::FieldReader<Udctrl>;
impl UdctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Udctrl> {
        match self.bits {
            0 => Some(Udctrl::Regular),
            1 => Some(Udctrl::Fcevent),
            2 => Some(Udctrl::Framestart),
            _ => None,
        }
    }
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == Udctrl::Regular
    }
    #[doc = "The data transfer is done at the next event triggered by the Frame Counter"]
    #[inline(always)]
    pub fn is_fcevent(&self) -> bool {
        *self == Udctrl::Fcevent
    }
    #[doc = "The data transfer is done continuously at every LCD frame start"]
    #[inline(always)]
    pub fn is_framestart(&self) -> bool {
        *self == Udctrl::Framestart
    }
}
#[doc = "Field `UDCTRL` writer - Update Data Control"]
pub type UdctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Udctrl>;
impl<'a, REG> UdctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(Udctrl::Regular)
    }
    #[doc = "The data transfer is done at the next event triggered by the Frame Counter"]
    #[inline(always)]
    pub fn fcevent(self) -> &'a mut crate::W<REG> {
        self.variant(Udctrl::Fcevent)
    }
    #[doc = "The data transfer is done continuously at every LCD frame start"]
    #[inline(always)]
    pub fn framestart(self) -> &'a mut crate::W<REG> {
        self.variant(Udctrl::Framestart)
    }
}
#[doc = "Field `DSC` reader - Direct Segment Control"]
pub type DscR = crate::BitReader;
#[doc = "Field `DSC` writer - Direct Segment Control"]
pub type DscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    pub fn udctrl(&self) -> UdctrlR {
        UdctrlR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 23 - Direct Segment Control"]
    #[inline(always)]
    pub fn dsc(&self) -> DscR {
        DscR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    pub fn udctrl(&mut self) -> UdctrlW<'_, CtrlSpec> {
        UdctrlW::new(self, 1)
    }
    #[doc = "Bit 23 - Direct Segment Control"]
    #[inline(always)]
    pub fn dsc(&mut self) -> DscW<'_, CtrlSpec> {
        DscW::new(self, 23)
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
