#[doc = "Register `TXDOUBLE` reader"]
pub type R = crate::R<TxdoubleSpec>;
#[doc = "Register `TXDOUBLE` writer"]
pub type W = crate::W<TxdoubleSpec>;
#[doc = "Field `TXDATA0` reader - TX Data"]
pub type Txdata0R = crate::FieldReader;
#[doc = "Field `TXDATA0` writer - TX Data"]
pub type Txdata0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXDATA1` reader - TX Data"]
pub type Txdata1R = crate::FieldReader;
#[doc = "Field `TXDATA1` writer - TX Data"]
pub type Txdata1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&self) -> Txdata0R {
        Txdata0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&self) -> Txdata1R {
        Txdata1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&mut self) -> Txdata0W<'_, TxdoubleSpec> {
        Txdata0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&mut self) -> Txdata1W<'_, TxdoubleSpec> {
        Txdata1W::new(self, 8)
    }
}
#[doc = "Transmit Buffer Double Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdouble::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdouble::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdoubleSpec;
impl crate::RegisterSpec for TxdoubleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdouble::R`](R) reader structure"]
impl crate::Readable for TxdoubleSpec {}
#[doc = "`write(|w| ..)` method takes [`txdouble::W`](W) writer structure"]
impl crate::Writable for TxdoubleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDOUBLE to value 0"]
impl crate::Resettable for TxdoubleSpec {}
