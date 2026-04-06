#[doc = "Register `USBCTRL` reader"]
pub type R = crate::R<UsbctrlSpec>;
#[doc = "Register `USBCTRL` writer"]
pub type W = crate::W<UsbctrlSpec>;
#[doc = "USB Rate Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbclksel {
    #[doc = "0: USHFRCO (clock recovery) is clocking USB"]
    Ushfrco = 0,
    #[doc = "1: HFXO clock is used to clock USB"]
    Hfxo = 1,
    #[doc = "2: HFXO clock doubler is used to clock USB"]
    Hfxox2 = 2,
    #[doc = "3: HFRCO clock is used to clock USB"]
    Hfrco = 3,
    #[doc = "4: LFXO clock is used to clock USB"]
    Lfxo = 4,
    #[doc = "5: LFRCO clock is used to clock USB"]
    Lfrco = 5,
}
impl From<Usbclksel> for u8 {
    #[inline(always)]
    fn from(variant: Usbclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbclksel {
    type Ux = u8;
}
impl crate::IsEnum for Usbclksel {}
#[doc = "Field `USBCLKSEL` reader - USB Rate Clock Select"]
pub type UsbclkselR = crate::FieldReader<Usbclksel>;
impl UsbclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Usbclksel> {
        match self.bits {
            0 => Some(Usbclksel::Ushfrco),
            1 => Some(Usbclksel::Hfxo),
            2 => Some(Usbclksel::Hfxox2),
            3 => Some(Usbclksel::Hfrco),
            4 => Some(Usbclksel::Lfxo),
            5 => Some(Usbclksel::Lfrco),
            _ => None,
        }
    }
    #[doc = "USHFRCO (clock recovery) is clocking USB"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == Usbclksel::Ushfrco
    }
    #[doc = "HFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Usbclksel::Hfxo
    }
    #[doc = "HFXO clock doubler is used to clock USB"]
    #[inline(always)]
    pub fn is_hfxox2(&self) -> bool {
        *self == Usbclksel::Hfxox2
    }
    #[doc = "HFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == Usbclksel::Hfrco
    }
    #[doc = "LFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Usbclksel::Lfxo
    }
    #[doc = "LFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Usbclksel::Lfrco
    }
}
#[doc = "Field `USBCLKSEL` writer - USB Rate Clock Select"]
pub type UsbclkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Usbclksel>;
impl<'a, REG> UsbclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USHFRCO (clock recovery) is clocking USB"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Usbclksel::Ushfrco)
    }
    #[doc = "HFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Usbclksel::Hfxo)
    }
    #[doc = "HFXO clock doubler is used to clock USB"]
    #[inline(always)]
    pub fn hfxox2(self) -> &'a mut crate::W<REG> {
        self.variant(Usbclksel::Hfxox2)
    }
    #[doc = "HFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Usbclksel::Hfrco)
    }
    #[doc = "LFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Usbclksel::Lfxo)
    }
    #[doc = "LFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Usbclksel::Lfrco)
    }
}
#[doc = "Field `USBCLKEN` reader - USB Rate Clock Enable"]
pub type UsbclkenR = crate::BitReader;
#[doc = "Field `USBCLKEN` writer - USB Rate Clock Enable"]
pub type UsbclkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - USB Rate Clock Select"]
    #[inline(always)]
    pub fn usbclksel(&self) -> UsbclkselR {
        UsbclkselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - USB Rate Clock Enable"]
    #[inline(always)]
    pub fn usbclken(&self) -> UsbclkenR {
        UsbclkenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Rate Clock Select"]
    #[inline(always)]
    pub fn usbclksel(&mut self) -> UsbclkselW<'_, UsbctrlSpec> {
        UsbclkselW::new(self, 0)
    }
    #[doc = "Bit 7 - USB Rate Clock Enable"]
    #[inline(always)]
    pub fn usbclken(&mut self) -> UsbclkenW<'_, UsbctrlSpec> {
        UsbclkenW::new(self, 7)
    }
}
#[doc = "USB Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbctrlSpec;
impl crate::RegisterSpec for UsbctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbctrl::R`](R) reader structure"]
impl crate::Readable for UsbctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbctrl::W`](W) writer structure"]
impl crate::Writable for UsbctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBCTRL to value 0"]
impl crate::Resettable for UsbctrlSpec {}
